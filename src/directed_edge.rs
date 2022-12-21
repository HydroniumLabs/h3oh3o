use crate::{delegate_inner, CellBoundary, H3Error, H3ErrorCodes, H3Index};
use h3o::{CellIndex, DirectedEdgeIndex};
use std::ffi::c_int;

/// Returns whether or not the provided H3Indexes are neighbors.
///
/// @param origin The origin H3 index.
/// @param destination The destination H3 index.
/// @param out Set to 1 if the indexes are neighbors, 0 otherwise
///
/// @return Error code if the origin or destination are invalid or incomparable.
#[no_mangle]
pub extern "C" fn areNeighborCells(
    origin: H3Index,
    destination: H3Index,
    out: Option<&mut c_int>,
) -> H3Error {
    fn inner(origin: H3Index, destination: H3Index) -> Result<c_int, H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let destination = CellIndex::try_from(destination)?;

        Ok(origin
            .is_neighbor_with(destination)
            .map(Into::into)
            .unwrap_or_default())
    }

    delegate_inner!(inner(origin, destination), out)
}

/// Returns a directed edge H3 index based on the provided origin and
/// destination
///
/// @param origin The origin H3 hexagon index
/// @param destination The destination H3 hexagon index
///
/// @return The directed edge H3Index, or H3_NULL on failure.
#[no_mangle]
pub extern "C" fn cellsToDirectedEdge(
    origin: H3Index,
    destination: H3Index,
    out: Option<&mut H3Index>,
) -> H3Error {
    fn inner(
        origin: H3Index,
        destination: H3Index,
    ) -> Result<H3Index, H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let destination = CellIndex::try_from(destination)?;

        Ok(origin
            .edge(destination)
            .ok_or(H3ErrorCodes::ENotNeighbors)?
            .into())
    }

    delegate_inner!(inner(origin, destination), out)
}

/// Provides the coordinates defining the directed edge.
///
/// @param edge The directed edge H3Index
/// @param cb The cellboundary object to store the edge coordinates.
#[no_mangle]
pub extern "C" fn directedEdgeToBoundary(
    edge: H3Index,
    gb: Option<&mut CellBoundary>,
) -> H3Error {
    fn inner(edge: H3Index) -> Result<CellBoundary, H3Error> {
        let index = DirectedEdgeIndex::try_from(edge)?;
        Ok(index.boundary().into())
    }

    delegate_inner!(inner(edge), gb)
}

/// Returns the origin, destination pair of hexagon IDs for the given edge ID
///
/// @param edge The directed edge H3Index
/// @param originDestination Pointer to memory to store origin and destination
/// IDs
///
/// # Safety
///
/// `originDestination` must points to an array of at least 2 elements.
#[no_mangle]
pub unsafe extern "C" fn directedEdgeToCells(
    edge: H3Index,
    originDestination: *mut H3Index,
) -> H3Error {
    fn inner(edge: H3Index) -> Result<(H3Index, H3Index), H3Error> {
        let index = DirectedEdgeIndex::try_from(edge)?;
        let (origin, destination) = index.cells();

        Ok((origin.into(), destination.into()))
    }

    match inner(edge) {
        Ok((origin, destination)) => {
            let slice = std::slice::from_raw_parts_mut(originDestination, 2);
            slice[0] = origin;
            slice[1] = destination;
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// Length of a directed edge in kilometers.
#[no_mangle]
pub extern "C" fn edgeLengthKm(
    edge: H3Index,
    length: Option<&mut f64>,
) -> H3Error {
    fn inner(edge: H3Index) -> Result<f64, H3Error> {
        let index = DirectedEdgeIndex::try_from(edge)?;
        Ok(index.length_km())
    }

    delegate_inner!(inner(edge), length)
}

/// Length of a directed edge in meters.
#[no_mangle]
pub extern "C" fn edgeLengthM(
    edge: H3Index,
    length: Option<&mut f64>,
) -> H3Error {
    fn inner(edge: H3Index) -> Result<f64, H3Error> {
        let index = DirectedEdgeIndex::try_from(edge)?;
        Ok(index.length_m())
    }

    delegate_inner!(inner(edge), length)
}

/// Length of a directed edge in radians.
///
/// @param   edge  H3 directed edge
/// @return        length in radians
#[no_mangle]
pub extern "C" fn edgeLengthRads(
    edge: H3Index,
    length: Option<&mut f64>,
) -> H3Error {
    fn inner(edge: H3Index) -> Result<f64, H3Error> {
        let index = DirectedEdgeIndex::try_from(edge)?;
        Ok(index.length_rads())
    }

    delegate_inner!(inner(edge), length)
}

/// Returns the destination hexagon from the directed edge H3Index
///
/// @param edge The edge H3 index
/// @return The destination H3 hexagon index, or H3_NULL on failure
#[no_mangle]
pub extern "C" fn getDirectedEdgeDestination(
    edge: H3Index,
    out: Option<&mut H3Index>,
) -> H3Error {
    fn inner(edge: H3Index) -> Result<H3Index, H3Error> {
        let index = DirectedEdgeIndex::try_from(edge)?;
        Ok(index.destination().into())
    }

    delegate_inner!(inner(edge), out)
}

/// Returns the origin hexagon from the directed edge H3Index
///
/// @param edge The edge H3 index
/// @return The origin H3 hexagon index, or H3_NULL on failure
#[no_mangle]
pub extern "C" fn getDirectedEdgeOrigin(
    edge: H3Index,
    out: Option<&mut H3Index>,
) -> H3Error {
    fn inner(edge: H3Index) -> Result<H3Index, H3Error> {
        let index = DirectedEdgeIndex::try_from(edge)?;
        Ok(index.origin().into())
    }

    delegate_inner!(inner(edge), out)
}

/// Determines if the provided H3Index is a valid directed edge index
///
/// @param edge The directed edge H3Index
/// @return 1 if it is a directed edge H3Index, otherwise 0.
#[no_mangle]
pub extern "C" fn isValidDirectedEdge(edge: H3Index) -> c_int {
    DirectedEdgeIndex::try_from(edge).is_ok().into()
}

/// Returns the 6 (or 5 for pentagons) edges associated with the H3Index.
///
/// # Safety
///
/// `edges` must points to an array of at least `6` elements (`5` for
/// pentagons),
#[no_mangle]
pub unsafe extern "C" fn originToDirectedEdges(
    origin: H3Index,
    edges: *mut H3Index,
) -> H3Error {
    fn inner(
        origin: H3Index,
    ) -> Result<(usize, impl Iterator<Item = DirectedEdgeIndex>), H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let len = if origin.is_pentagon() { 5 } else { 6 };
        Ok((len, origin.edges()))
    }

    match inner(origin) {
        Ok((len, iter)) => {
            let slice = std::slice::from_raw_parts_mut(edges, len);
            for (i, edge) in iter.enumerate() {
                slice[i] = edge.into();
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}
