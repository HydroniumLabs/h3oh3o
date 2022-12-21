use crate::{H3Error, H3ErrorCodes, H3Index};
use h3o::{CellIndex, Resolution};
use std::ffi::c_int;

pub fn h3res_to_resolution(res: c_int) -> Result<Resolution, H3ErrorCodes> {
    let res = u8::try_from(res).map_err(|_| H3ErrorCodes::EResDomain)?;
    res.try_into().map_err(|_| H3ErrorCodes::EResDomain)
}

/// Cast a C-array (ptr + len) of `H3Index` into a slice of `CellIndex`.
///
/// # Safety
///
/// `ptr` must points to an array of at least `len` elements.
pub unsafe fn h3ptr_to_h3oslice<'a>(
    ptr: *const H3Index,
    len: i64,
) -> Result<&'a [CellIndex], H3Error> {
    let len = usize::try_from(len).expect("H3Index array too large");
    let indexes = std::slice::from_raw_parts(ptr, len);

    if !indexes
        .iter()
        .all(|&index| CellIndex::try_from(index).is_ok())
    {
        return Err(H3ErrorCodes::ECellInvalid.into());
    }

    // Ok, every provided cell index is valid.
    // Cast to avoid a copy (safe because CellIndex is repr(tranparent)).
    Ok(&*(indexes as *const [H3Index] as *const [CellIndex]))
}

/// Cast a C-array (ptr + len) of `H3Index` into a slice of `CellIndex`.
///
/// # Safety
///
/// `ptr` must points to an array of at least `len` elements.
pub unsafe fn h3ptr_to_h3oslice_mut<'a>(
    ptr: *mut H3Index,
    len: c_int,
) -> Result<&'a mut [CellIndex], H3Error> {
    let len = usize::try_from(len).expect("H3Index array too large");
    let indexes = std::slice::from_raw_parts_mut(ptr, len);

    if !indexes
        .iter()
        .all(|&index| CellIndex::try_from(index).is_ok())
    {
        return Err(H3ErrorCodes::ECellInvalid.into());
    }

    // Ok, every provided cell index is valid.
    // Cast to avoid a copy (safe because CellIndex is repr(tranparent)).
    Ok(&mut *(indexes as *mut [H3Index] as *mut [CellIndex]))
}
