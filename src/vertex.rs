use crate::{delegate_inner, H3Error, H3ErrorCodes, H3Index, LatLng};
use h3o::{CellIndex, VertexIndex};
use std::ffi::c_int;

/// Get a single vertex for a given cell, as an H3 index, or
/// H3_NULL if the vertex is invalid
///
/// @param cell    Cell to get the vertex for
/// @param vertexNum Number (index) of the vertex to calculate
#[no_mangle]
pub extern "C" fn cellToVertex(
    origin: H3Index,
    vertexNum: c_int,
    out: Option<&mut H3Index>,
) -> H3Error {
    fn inner(origin: H3Index, vertexNum: c_int) -> Result<H3Index, H3Error> {
        let index = CellIndex::try_from(origin)?;
        let vertexNum = u8::try_from(vertexNum)
            .map_err(|_| H3ErrorCodes::EDomain)?
            .try_into()?;

        Ok(index.vertex(vertexNum).ok_or(H3ErrorCodes::EDomain)?.into())
    }

    delegate_inner!(inner(origin, vertexNum), out)
}

/// Get all vertexes for the given cell
///
/// @param cell      Cell to get the vertexes for
/// @param vertexes  Array to hold vertex output.
///
/// # Safety
///
/// `vertexes` must points to an array of at least 6 elements.
#[no_mangle]
pub unsafe extern "C" fn cellToVertexes(
    origin: H3Index,
    vertexes: *mut H3Index,
) -> H3Error {
    fn inner(
        origin: H3Index,
    ) -> Result<impl Iterator<Item = VertexIndex>, H3Error> {
        let origin = CellIndex::try_from(origin)?;
        Ok(origin.vertexes())
    }

    match inner(origin) {
        Ok(iter) => {
            let slice = std::slice::from_raw_parts_mut(vertexes, 6);
            for (i, index) in iter.enumerate() {
                slice[i] = index.into();
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// Whether the input is a valid H3 vertex
///
/// @param  vertex H3 index possibly describing a vertex
/// @return        Whether the input is valid
#[no_mangle]
pub extern "C" fn isValidVertex(vertex: H3Index) -> c_int {
    VertexIndex::try_from(vertex).is_ok().into()
}

/// Get the geocoordinates of an H3 vertex
///
/// @param vertex H3 index describing a vertex
/// @param coord  Output geo coordinate
#[no_mangle]
pub extern "C" fn vertexToLatLng(
    vertex: H3Index,
    point: Option<&mut LatLng>,
) -> H3Error {
    fn inner(vertex: H3Index) -> Result<LatLng, H3Error> {
        Ok(h3o::LatLng::from(VertexIndex::try_from(vertex)?).into())
    }

    delegate_inner!(inner(vertex), point)
}
