use crate::{
    convert, delegate_inner, CellBoundary, H3Error, H3ErrorCodes, H3Index,
    LatLng,
};
use h3o::CellIndex;
use std::ffi::c_int;

/// Area of H3 cell in kilometers^2.
#[no_mangle]
pub extern "C" fn cellAreaKm2(h: H3Index, out: Option<&mut f64>) -> H3Error {
    fn inner(h: H3Index) -> Result<f64, H3Error> {
        let index = CellIndex::try_from(h)?;
        Ok(index.area_km2())
    }

    delegate_inner!(inner(h), out)
}

/// Area of H3 cell in meters^2.
#[no_mangle]
pub extern "C" fn cellAreaM2(h: H3Index, out: Option<&mut f64>) -> H3Error {
    fn inner(h: H3Index) -> Result<f64, H3Error> {
        let index = CellIndex::try_from(h)?;
        Ok(index.area_m2())
    }

    delegate_inner!(inner(h), out)
}

/// Area of H3 cell in radians^2.
///
/// The area is calculated by breaking the cell into spherical triangles and
/// summing up their areas. Note that some H3 cells (hexagons and pentagons)
/// are irregular, and have more than 6 or 5 sides.
///
/// todo: optimize the computation by re-using the edges shared between triangles
///
/// @param   cell  H3 cell
/// @param    out  cell area in radians^2
/// @return        E_SUCCESS on success, or an error code otherwise
#[no_mangle]
pub extern "C" fn cellAreaRads2(h: H3Index, out: Option<&mut f64>) -> H3Error {
    fn inner(h: H3Index) -> Result<f64, H3Error> {
        let index = CellIndex::try_from(h)?;
        Ok(index.area_rads2())
    }

    delegate_inner!(inner(h), out)
}

/// Determines the cell boundary in spherical coordinates for an H3 index.1
//
/// @param h3 The H3 index.
#[no_mangle]
pub extern "C" fn cellToBoundary(
    h3: H3Index,
    gp: Option<&mut CellBoundary>,
) -> H3Error {
    fn inner(h3: H3Index) -> Result<CellBoundary, H3Error> {
        let index = CellIndex::try_from(h3)?;
        Ok(index.boundary().into())
    }

    delegate_inner!(inner(h3), gp)
}

/// cellToCenterChild produces the center child index for a given H3 index at
/// the specified resolution
///
/// @param h H3Index to find center child of
/// @param childRes The resolution to switch to
/// @param child H3Index of the center child
/// @return 0 (E_SUCCESS) on success
#[no_mangle]
pub extern "C" fn cellToCenterChild(
    h: H3Index,
    childRes: c_int,
    child: Option<&mut H3Index>,
) -> H3Error {
    fn inner(h: H3Index, childRes: c_int) -> Result<H3Index, H3Error> {
        let index = CellIndex::try_from(h)?;
        let child_res = convert::h3res_to_resolution(childRes)?;
        Ok(index
            .center_child(child_res)
            .ok_or(H3ErrorCodes::EResDomain)?
            .into())
    }

    delegate_inner!(inner(h, childRes), child)
}

/// cellToChildren takes the given hexagon id and generates all of the children
/// at the specified resolution storing them into the provided memory pointer.
/// It's assumed that cellToChildrenSize was used to determine the allocation.
///
/// @param h H3Index to find the children of
/// @param childRes int the child level to produce
/// @param children H3Index* the memory to store the resulting addresses in
///
/// # Safety
///
/// `children` must points to an array of at least `cellToChildrenSize(h,
/// childRes)` elements.
#[no_mangle]
pub unsafe extern "C" fn cellToChildren(
    h: H3Index,
    childRes: c_int,
    children: *mut H3Index,
) -> H3Error {
    fn inner(
        h: H3Index,
        childRes: c_int,
    ) -> Result<(u64, impl Iterator<Item = CellIndex>), H3Error> {
        let index = CellIndex::try_from(h)?;
        let child_res = convert::h3res_to_resolution(childRes)?;
        Ok((index.children_count(child_res), index.children(child_res)))
    }

    match inner(h, childRes) {
        Ok((len, iter)) => {
            let len = usize::try_from(len).expect("overflow");
            let slice = std::slice::from_raw_parts_mut(children, len);
            for (i, child) in iter.enumerate() {
                slice[i] = child.into();
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// cellToChildrenSize returns the exact number of children for a cell at a
/// given child resolution.
///
/// @param h         H3Index to find the number of children of
/// @param childRes  The child resolution you're interested in
///
/// @return int      Exact number of children (handles hexagons and pentagons
///                  correctly)
#[no_mangle]
pub extern "C" fn cellToChildrenSize(
    h: H3Index,
    childRes: c_int,
    out: Option<&mut i64>,
) -> H3Error {
    fn inner(h: H3Index, childRes: c_int) -> Result<i64, H3Error> {
        // Ideally, should return ECellInvalid, but H3 only return res error.
        let index =
            CellIndex::try_from(h).map_err(|_| H3ErrorCodes::EResDomain)?;
        let child_res = convert::h3res_to_resolution(childRes)?;
        Ok(i64::try_from(index.children_count(child_res)).expect("overflow"))
    }

    delegate_inner!(inner(h, childRes), out)
}

/// Determines the spherical coordinates of the center point of an H3 index.
///
/// @param h3 The H3 index.
#[no_mangle]
pub extern "C" fn cellToLatLng(h3: H3Index, g: Option<&mut LatLng>) -> H3Error {
    fn inner(h3: H3Index) -> Result<LatLng, H3Error> {
        let index = CellIndex::try_from(h3)?;
        Ok(h3o::LatLng::from(index).into())
    }

    delegate_inner!(inner(h3), g)
}

/// cellToParent produces the parent index for a given H3 index
///
/// @param h H3Index to find parent of
/// @param parentRes The resolution to switch to (parent, grandparent, etc)
///
#[no_mangle]
pub extern "C" fn cellToParent(
    h: H3Index,
    parentRes: c_int,
    parent: Option<&mut H3Index>,
) -> H3Error {
    fn inner(h: H3Index, parentRes: c_int) -> Result<H3Index, H3Error> {
        let index = CellIndex::try_from(h)?;
        let parent_res = convert::h3res_to_resolution(parentRes)?;
        Ok(index
            .parent(parent_res)
            .map(Into::into)
            .ok_or(h3o::error::ResolutionMismatch)?)
    }

    delegate_inner!(inner(h, parentRes), parent)
}

/// Returns the H3 base cell "number" of an H3 cell (hexagon or pentagon).
///
/// @param h The H3 cell.
/// @return The base cell "number" of the H3 cell argument.
#[no_mangle]
pub extern "C" fn getBaseCellNumber(h: H3Index) -> c_int {
    CellIndex::try_from(h)
        .map(CellIndex::base_cell)
        .map(|cell| u8::from(cell).into())
        .unwrap_or(-1)
}

/// Find all icosahedron faces intersected by a given H3 index, represented
/// as integers from 0-19. The array is sparse; since 0 is a valid value,
/// invalid array values are represented as -1. It is the responsibility of
/// the caller to filter out invalid values.
///
/// @param h3 The H3 index
/// @param out Output array.
///
/// # Safety
///
/// `out` must points to an array of at least `maxFaceCount(h3)` elements.
#[no_mangle]
pub unsafe extern "C" fn getIcosahedronFaces(
    h3: H3Index,
    out: *mut c_int,
) -> H3Error {
    fn inner(h3: H3Index) -> Result<(usize, h3o::FaceSet), H3Error> {
        let index = CellIndex::try_from(h3)?;
        Ok((index.max_face_count(), index.icosahedron_faces()))
    }

    match inner(h3) {
        Ok((len, faces)) => {
            let slice = std::slice::from_raw_parts_mut(out, len);
            // H3 returns a sparse array, so we must fill it with invalid values
            // to mark unused slots.
            slice.fill(-1);
            for (i, face) in faces.iter().enumerate() {
                slice[i] = u8::from(face).into();
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// Returns the H3 resolution of an H3 index.
/// @param h The H3 index.
/// @return The resolution of the H3 index argument.
#[no_mangle]
pub extern "C" fn getResolution(h: H3Index) -> c_int {
    CellIndex::try_from(h)
        .map(|index| u8::from(index.resolution()).into())
        .unwrap_or(33)
}

/// isPentagon takes an H3Index and determines if it is actually a pentagon.
/// @param h The H3Index to check.
/// @return Returns 1 if it is a pentagon, otherwise 0.
#[no_mangle]
pub extern "C" fn isPentagon(h: H3Index) -> c_int {
    CellIndex::try_from(h)
        .map(CellIndex::is_pentagon)
        .unwrap_or_default()
        .into()
}

/// Returns whether or not an H3 index is a valid cell (hexagon or pentagon).
/// @param h The H3 index to validate.
/// @return 1 if the H3 index if valid, and 0 if it is not.
#[no_mangle]
pub extern "C" fn isValidCell(h: H3Index) -> c_int {
    CellIndex::try_from(h).is_ok().into()
}

/// Returns the max number of possible icosahedron faces an H3 index
/// may intersect.
///
/// @return int count of faces
#[no_mangle]
pub extern "C" fn maxFaceCount(
    h3: H3Index,
    out: Option<&mut c_int>,
) -> H3Error {
    fn inner(h3: H3Index) -> Result<c_int, H3Error> {
        let index = CellIndex::try_from(h3)?;
        Ok(c_int::try_from(index.max_face_count()).expect("5 or 2"))
    }

    delegate_inner!(inner(h3), out)
}

/// Returns the position of the cell within an ordered list of all children of
/// the cell's parent at the specified resolution.
#[no_mangle]
pub extern "C" fn cellToChildPos(
    child: H3Index,
    parentRes: c_int,
    out: Option<&mut i64>,
) -> H3Error {
    fn inner(child: H3Index, parentRes: c_int) -> Result<i64, H3Error> {
        let index = CellIndex::try_from(child)
            .map_err(|_| H3ErrorCodes::ECellInvalid)?;
        let parent_res = convert::h3res_to_resolution(parentRes)?;
        let position = index
            .child_position(parent_res)
            .ok_or(H3ErrorCodes::EResMismatch)?;
        Ok(position.try_into().expect("overflow"))
    }

    delegate_inner!(inner(child, parentRes), out)
}

/// Returns the child cell at a given position within an ordered list of all
/// children at the specified resolution.
#[no_mangle]
pub extern "C" fn childPosToCell(
    childPos: i64,
    parent: H3Index,
    childRes: c_int,
    out: Option<&mut H3Index>,
) -> H3Error {
    fn inner(
        childPos: i64,
        parent: H3Index,
        childRes: c_int,
    ) -> Result<H3Index, H3Error> {
        let index = CellIndex::try_from(parent)
            .map_err(|_| H3ErrorCodes::ECellInvalid)?;
        let child_pos =
            u64::try_from(childPos).map_err(|_| H3ErrorCodes::EDomain)?;
        let child_res = convert::h3res_to_resolution(childRes)?;
        let child = index
            .child_at(child_pos, child_res)
            .ok_or(H3ErrorCodes::EResMismatch)?;
        Ok(child.into())
    }

    delegate_inner!(inner(childPos, parent, childRes), out)
}
