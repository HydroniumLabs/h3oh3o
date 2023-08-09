use crate::{convert, delegate_inner, H3Error, H3ErrorCodes, H3Index, LatLng};
use geo_types::{Coord, LineString, MultiPolygon, Polygon};
use h3o::geom::{PolyfillConfig, Polygon as h3oPolygon, ToCells, ToGeo};
use std::{ffi::c_int, ptr};

/// Create a LinkedGeoPolygon describing the outline(s) of a set of  hexagons.
/// Polygon outlines will follow GeoJSON MultiPolygon order: Each polygon will
/// have one outer loop, which is first in the list, followed by any holes.
///
/// It is the responsibility of the caller to call destroyLinkedMultiPolygon on
/// the populated linked geo structure, or the memory for that structure will
/// not be freed.
///
/// It is expected that all hexagons in the set have the same resolution and
/// that the set contains no duplicates. Behavior is undefined if duplicates
/// or multiple resolutions are present, and the algorithm may produce
/// unexpected or invalid output.
///
/// @param h3Set    Set of hexagons
/// @param numHexes Number of hexagons in set
/// @param out      Output polygon
///
/// # Safety
///
/// `h3Set` must points to an array of at least `numHexes` elements.
#[no_mangle]
pub unsafe extern "C" fn cellsToLinkedMultiPolygon(
    h3Set: *const H3Index,
    numHexes: c_int,
    out: Option<&mut LinkedGeoPolygon>,
) -> H3Error {
    unsafe fn inner(
        h3Set: *const H3Index,
        numHexes: c_int,
    ) -> Result<LinkedGeoPolygon, H3Error> {
        let indexes = convert::h3ptr_to_h3oslice(h3Set, numHexes.into())?;
        Ok(indexes.iter().copied().to_geom(false)?.into())
    }
    if numHexes == 0 {
        *out.expect("null pointer") = LinkedGeoPolygon {
            first: ptr::null_mut(),
            last: ptr::null_mut(),
            next: ptr::null_mut(),
        };
        return H3ErrorCodes::ESuccess.into();
    }

    match inner(h3Set, numHexes) {
        Ok(polygon) => {
            *out.expect("null pointer") = polygon;
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// Free all allocated memory for a linked geo structure. The caller is
/// responsible for freeing memory allocated to input polygon struct.
///
/// @param polygon Pointer to the first polygon in the structure
///
/// # Safety
///
/// The pointer must comes from [`cellsToLinkedMultiPolygon`]
#[no_mangle]
pub unsafe extern "C" fn destroyLinkedMultiPolygon(
    polygon: Option<&mut LinkedGeoPolygon>,
) {
    if let Some(polygon) = polygon {
        let mut curr_polygon = *polygon;
        // For each polygon.
        loop {
            // Free the rings.
            let mut curr_ring = curr_polygon.first;
            while !curr_ring.is_null() {
                let next_ring = (*curr_ring).next;

                // Free the coordinates.
                let mut curr_coord = (*curr_ring).first;
                while !curr_coord.is_null() {
                    let next_coord = (*curr_coord).next;
                    drop(Box::from_raw(curr_coord));
                    curr_coord = next_coord;
                }

                drop(Box::from_raw(curr_ring));
                curr_ring = next_ring;
            }
            // We're done?
            if curr_polygon.next.is_null() {
                break;
            }
            // Still here? On to the next!
            curr_polygon = *Box::from_raw(curr_polygon.next);
        }
    }
}

/// maxPolygonToCellsSize returns the number of cells to allocate space for
/// when performing a polygonToCells on the given GeoJSON-like data structure.
///
/// The size is the maximum of either the number of points in the geoloop or the
/// number of cells in the bounding box of the geoloop.
///
/// @param geoPolygon A GeoJSON-like data structure indicating the poly to fill
/// @param res Hexagon resolution (0-15)
/// @param out number of cells to allocate for
/// @return 0 (E_SUCCESS) on success.
#[no_mangle]
pub extern "C" fn maxPolygonToCellsSize(
    geoPolygon: Option<&GeoPolygon>,
    res: c_int,
    flags: u32,
    out: Option<&mut i64>,
) -> H3Error {
    fn inner(
        geoPolygon: &GeoPolygon,
        res: c_int,
        flags: u32,
    ) -> Result<i64, H3Error> {
        if flags != 0 {
            return Err(H3ErrorCodes::EOptionInvalid.into());
        }
        // Empty polygon contains no cell.
        if geoPolygon.geoloop.numVerts == 0 {
            return Ok(0);
        }

        let resolution = convert::h3res_to_resolution(res)?;
        let polygon = Polygon::try_from(*geoPolygon)?;
        let polygon = h3oPolygon::from_radians(polygon)?;

        Ok(polygon
            .max_cells_count(PolyfillConfig::new(resolution))
            .try_into()
            .expect("too many cells"))
    }

    geoPolygon.map_or_else(
        || H3ErrorCodes::EFailed.into(),
        |geoPolygon| delegate_inner!(inner(geoPolygon, res, flags), out),
    )
}

/// polygonToCells takes a given GeoJSON-like data structure and preallocated,
/// zeroed memory, and fills it with the hexagons that are contained by
/// the GeoJSON-like data structure.
///
/// This implementation traces the GeoJSON geoloop(s) in cartesian space with
/// hexagons, tests them and their neighbors to be contained by the geoloop(s),
/// and then any newly found hexagons are used to test again until no new
/// hexagons are found.
///
/// @param geoPolygon The geoloop and holes defining the relevant area
/// @param res The Hexagon resolution (0-15)
/// @param out The slab of zeroed memory to write to. Assumed to be big enough.
///
/// # Safety
///
/// `out` must points to an array of at least `maxPolygonToCellsSize` elements.
#[no_mangle]
pub unsafe extern "C" fn polygonToCells(
    geoPolygon: Option<&GeoPolygon>,
    res: c_int,
    flags: u32,
    out: *mut H3Index,
) -> H3Error {
    unsafe fn inner(
        geoPolygon: &GeoPolygon,
        res: c_int,
        flags: u32,
        out: *mut H3Index,
    ) -> Result<(), H3Error> {
        if flags != 0 {
            return Err(H3ErrorCodes::EOptionInvalid.into());
        }
        let resolution = convert::h3res_to_resolution(res)?;

        // Empty polygon contains no cell.
        if geoPolygon.geoloop.numVerts == 0 {
            return Ok(());
        }

        let polygon = Polygon::try_from(*geoPolygon)?;
        let polygon = h3oPolygon::from_radians(polygon)?;
        let config = PolyfillConfig::new(resolution);
        let len = polygon.max_cells_count(config);
        let cells = polygon.to_cells(config);

        let out = std::slice::from_raw_parts_mut(out, len);
        for (i, cell_index) in cells.enumerate() {
            out[i] = cell_index.into();
        }
        Ok(())
    }

    geoPolygon.map_or_else(
        || H3ErrorCodes::EFailed.into(),
        |geoPolygon| {
            inner(geoPolygon, res, flags, out)
                .err()
                .unwrap_or_else(|| H3ErrorCodes::ESuccess.into())
        },
    )
}

// -----------------------------------------------------------------------------

/// Similar to `CellBoundary`, but requires more alloc work.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GeoLoop {
    pub numVerts: c_int,
    pub verts: *mut LatLng,
}

impl TryFrom<GeoLoop> for LineString<f64> {
    type Error = H3Error;

    fn try_from(value: GeoLoop) -> Result<Self, Self::Error> {
        let len = usize::try_from(value.numVerts)
            .map_err(|_| H3ErrorCodes::EFailed)?;
        // SAFETY: `verts` must points to an array of at least `numVerts`
        // elements.
        unsafe {
            let verts = std::slice::from_raw_parts_mut(value.verts, len);
            Ok(Self::new(
                verts.iter_mut().map(|ll| Coord::from(*ll)).collect(),
            ))
        }
    }
}

// -----------------------------------------------------------------------------

/// Simplified core of GeoJSON Polygon coordinates definition.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GeoPolygon {
    /// Exterior boundary of the polygon.
    pub geoloop: GeoLoop,
    /// Number of elements in the array pointed to by holes.
    pub numHoles: c_int,
    /// Interior boundaries (holes) in the polygon.
    pub holes: *mut GeoLoop,
}

impl TryFrom<GeoPolygon> for Polygon<f64> {
    type Error = H3Error;

    fn try_from(value: GeoPolygon) -> Result<Self, Self::Error> {
        let len = usize::try_from(value.numHoles)
            .map_err(|_| H3ErrorCodes::EFailed)?;
        // SAFETY: `holes` must points to an array of at least `numHoles`
        // elements.
        unsafe {
            let holes = std::slice::from_raw_parts_mut(value.holes, len);
            Ok(Self::new(
                value.geoloop.try_into()?,
                holes
                    .iter_mut()
                    .map(|hole| LineString::try_from(*hole))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
}

// -----------------------------------------------------------------------------

pub struct GeoMultiPolygon {
    pub num_polygons: c_int,
    pub polygons: *mut GeoPolygon,
}

// -----------------------------------------------------------------------------

/// A coordinate node in a linked geo structure, part of a linked list
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LinkedLatLng {
    pub vertex: LatLng,
    pub next: *mut Self,
}

impl From<Coord> for LinkedLatLng {
    fn from(value: Coord) -> Self {
        Self {
            vertex: LatLng::from(value),
            next: ptr::null_mut(),
        }
    }
}

// -----------------------------------------------------------------------------

/// A loop node in a linked geo structure, part of a linked list
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LinkedGeoLoop {
    pub first: *mut LinkedLatLng,
    pub last: *mut LinkedLatLng,
    pub next: *mut Self,
}

impl From<LineString> for LinkedGeoLoop {
    fn from(mut value: LineString) -> Self {
        let mut ring = Self {
            first: ptr::null_mut(),
            last: ptr::null_mut(),
            next: ptr::null_mut(),
        };

        // SAFETY: `last` is always set before being dereferenced.
        unsafe {
            // Our rings are closed (first point == last point) but this isn't
            // the case for H3. So remove the last point before the conversion.
            value.0.pop();
            for coord in value.into_inner() {
                let node = Box::into_raw(Box::new(coord.into()));
                if ring.last.is_null() {
                    ring.first = node;
                } else {
                    (*ring.last).next = node;
                }
                ring.last = node;
            }
        }

        ring
    }
}

// -----------------------------------------------------------------------------

/// A polygon node in a linked geo structure, part of a linked list.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LinkedGeoPolygon {
    pub first: *mut LinkedGeoLoop,
    pub last: *mut LinkedGeoLoop,
    pub next: *mut Self,
}

impl From<MultiPolygon> for LinkedGeoPolygon {
    fn from(value: MultiPolygon) -> Self {
        let mut head = ptr::null_mut();
        assert!(!value.0.is_empty(), "empty multipolygon");
        // SAFETY: we should always have at least 1 polygon in the multipolygon.
        unsafe {
            for polygon in value.0.into_iter().rev() {
                let mut node = Self::from(polygon);
                node.next = head;
                head = Box::into_raw(Box::new(node));
            }
            *head
        }
    }
}

impl From<Polygon> for LinkedGeoPolygon {
    fn from(value: Polygon) -> Self {
        let mut polygon = Self {
            first: ptr::null_mut(),
            last: ptr::null_mut(),
            next: ptr::null_mut(),
        };
        let (exterior, interiors) = value.into_inner();
        let rings = std::iter::once(exterior).chain(interiors.into_iter());

        // SAFETY: `last` is always set before being dereferenced.
        unsafe {
            for ring in rings {
                let node = Box::into_raw(Box::new(ring.into()));
                if polygon.last.is_null() {
                    polygon.first = node;
                } else {
                    (*polygon.last).next = node;
                }
                polygon.last = node;
            }
        }

        polygon
    }
}
