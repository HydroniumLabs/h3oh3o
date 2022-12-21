use crate::{convert, delegate_inner, H3Error, H3ErrorCodes, H3Index};
use h3o::CellIndex;
use std::ffi::c_int;

/// compactCells takes a set of hexagons all at the same resolution and
/// compresses them by pruning full child branches to the parent level. This is
/// also done for all parents recursively to get the minimum number of hex
/// addresses that perfectly cover the defined space.
///
/// @param h3Set        Set of hexagons
/// @param compactedSet The output array of compressed hexagons (preallocated)
/// @param numHexes     The size of the input and output arrays (possible that no
///                     contiguous regions exist in the set at all and no
///                     compression possible)
/// @return an error code on bad input data
///
/// # Safety
///
/// `h3Set` and `compactedSet` must points to an array of at least `numHexes`
/// elements.
#[no_mangle]
pub unsafe extern "C" fn compactCells(
    h3Set: *const H3Index,
    compactedSet: *mut H3Index,
    numHexes: i64,
) -> H3Error {
    unsafe fn inner(
        h3Set: *const H3Index,
        numHexes: i64,
    ) -> Result<impl Iterator<Item = CellIndex>, H3Error> {
        let indexes = convert::h3ptr_to_h3oslice(h3Set, numHexes)?;

        Ok(CellIndex::compact(indexes.iter().copied())?)
    }

    if numHexes == 0 {
        return H3ErrorCodes::ESuccess.into();
    }

    match inner(h3Set, numHexes) {
        Ok(iter) => {
            let len = usize::try_from(numHexes).expect("overflow");
            let slice = std::slice::from_raw_parts_mut(compactedSet, len);
            for (i, cell_index) in iter.enumerate() {
                slice[i] = cell_index.into();
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// uncompactCells takes a compressed set of cells and expands back to the
/// original set of cells.
///
/// @param   compactSet  Set of compacted cells
/// @param   numCompact  The number of cells in the input compacted set
/// @param   outSet      Output array for decompressed cells (preallocated)
/// @param   numOut      The size of the output array to bound check against
/// @param   res         The H3 resolution to decompress to
/// @return              An error code if output array is too small or any cell
///                       is smaller than the output resolution.
/// # Safety
///
/// `compactedSet` must points to an array of at least `numCompacted` elements.
/// `outSet` must points to an array of at least `numOut` elements.
#[no_mangle]
pub unsafe extern "C" fn uncompactCells(
    compactedSet: *const H3Index,
    numCompacted: i64,
    outSet: *mut H3Index,
    numOut: i64,
    res: c_int,
) -> H3Error {
    unsafe fn inner(
        compactedSet: *const H3Index,
        numCompacted: i64,
        res: c_int,
    ) -> Result<impl Iterator<Item = CellIndex>, H3Error> {
        let res = convert::h3res_to_resolution(res)?;
        let indexes = convert::h3ptr_to_h3oslice(compactedSet, numCompacted)?;

        Ok(CellIndex::uncompact(indexes.iter().copied(), res))
    }

    if numCompacted == 0 {
        return H3ErrorCodes::ESuccess.into();
    }

    match inner(compactedSet, numCompacted, res) {
        Ok(iter) => {
            let len = usize::try_from(numOut).expect("overflow");
            let slice = std::slice::from_raw_parts_mut(outSet, len);
            for (i, cell_index) in iter.enumerate() {
                slice[i] = cell_index.into();
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// uncompactCellsSize takes a compacted set of hexagons and provides
/// the exact size of the uncompacted set of hexagons.
///
/// @param   compactedSet  Set of hexagons
/// @param   numHexes      The number of hexes in the input set
/// @param   res           The hexagon resolution to decompress to
/// @param   out           The number of hexagons to allocate memory for
///
/// # Safety
///
/// `compactedSet` must points to an array of at least `numCompacted` elements.
#[no_mangle]
pub unsafe extern "C" fn uncompactCellsSize(
    compactedSet: *const H3Index,
    numCompacted: i64,
    res: c_int,
    out: Option<&mut i64>,
) -> H3Error {
    unsafe fn inner(
        compactedSet: *const H3Index,
        numCompacted: i64,
        res: c_int,
    ) -> Result<i64, H3Error> {
        let res = convert::h3res_to_resolution(res)?;
        let indexes = convert::h3ptr_to_h3oslice(compactedSet, numCompacted)?;

        Ok(CellIndex::uncompact_size(indexes.iter().copied(), res)
            .try_into()
            .expect("positive count"))
    }

    if numCompacted == 0 {
        *out.expect("null pointer") = 0;
        return H3ErrorCodes::ESuccess.into();
    }
    delegate_inner!(inner(compactedSet, numCompacted, res), out)
}
