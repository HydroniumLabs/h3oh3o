use crate::{convert, delegate_inner, H3Error, H3ErrorCodes, H3Index};
use h3o::{BaseCell, CellIndex, Resolution};
use std::ffi::c_int;

/// Average hexagon area in square kilometers (excludes pentagons).
#[no_mangle]
pub extern "C" fn getHexagonAreaAvgKm2(
    res: c_int,
    out: Option<&mut f64>,
) -> H3Error {
    fn inner(res: c_int) -> Result<f64, H3Error> {
        let res = convert::h3res_to_resolution(res)?;
        Ok(res.area_km2())
    }

    delegate_inner!(inner(res), out)
}

/// Average hexagon area in square meters (excludes pentagons).
#[no_mangle]
pub extern "C" fn getHexagonAreaAvgM2(
    res: c_int,
    out: Option<&mut f64>,
) -> H3Error {
    fn inner(res: c_int) -> Result<f64, H3Error> {
        let res = convert::h3res_to_resolution(res)?;
        Ok(res.area_m2())
    }

    delegate_inner!(inner(res), out)
}

/// Average hexagon edge length in kilometers (excludes pentagons).
#[no_mangle]
pub extern "C" fn getHexagonEdgeLengthAvgKm(
    res: c_int,
    out: Option<&mut f64>,
) -> H3Error {
    fn inner(res: c_int) -> Result<f64, H3Error> {
        let res = convert::h3res_to_resolution(res)?;
        Ok(res.edge_length_km())
    }

    delegate_inner!(inner(res), out)
}

/// Average hexagon edge length in meters (excludes pentagons).
#[no_mangle]
pub extern "C" fn getHexagonEdgeLengthAvgM(
    res: c_int,
    out: Option<&mut f64>,
) -> H3Error {
    fn inner(res: c_int) -> Result<f64, H3Error> {
        let res = convert::h3res_to_resolution(res)?;
        Ok(res.edge_length_m())
    }

    delegate_inner!(inner(res), out)
}

/// Number of cells (hexagons and pentagons) for a given resolution.
///
/// @param   res  H3 cell resolution
/// @return  number of cells at resolution `res`
#[no_mangle]
pub extern "C" fn getNumCells(res: c_int, out: Option<&mut i64>) -> H3Error {
    fn inner(res: c_int) -> Result<i64, H3Error> {
        let resolution = convert::h3res_to_resolution(res)?;
        let count = resolution.cell_count();
        Ok(i64::try_from(count).expect("cell count overflow"))
    }

    delegate_inner!(inner(res), out)
}

/// Generates all pentagons at the specified resolution
///
/// @param res The resolution to produce pentagons at.
/// @param out Output array.
///
/// # Safety
///
/// `out` must points to an array of at least `pentagonCount` elements.
#[no_mangle]
pub unsafe extern "C" fn getPentagons(
    res: c_int,
    out: *mut H3Index,
) -> H3Error {
    fn inner(
        res: c_int,
    ) -> Result<(usize, impl Iterator<Item = CellIndex>), H3Error> {
        let res = convert::h3res_to_resolution(res)?;
        Ok((Resolution::pentagon_count().into(), res.pentagons()))
    }

    match inner(res) {
        Ok((len, pentagons)) => {
            let slice = std::slice::from_raw_parts_mut(out, len);
            for (i, pentagon) in pentagons.enumerate() {
                slice[i] = pentagon.into();
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// getRes0Cells generates all base cells storing them into the provided
/// memory pointer.
///
/// @param out H3Index* the memory to store the resulting base cells in
/// @returns E_SUCCESS.
///
/// # Safety
///
/// `out` must points to an array of at least `res0CellCount` elements.
#[no_mangle]
pub unsafe extern "C" fn getRes0Cells(out: *mut H3Index) -> H3Error {
    let slice = std::slice::from_raw_parts_mut(out, BaseCell::count().into());
    for (i, cell) in CellIndex::base_cells().enumerate() {
        slice[i] = cell.into();
    }
    H3ErrorCodes::ESuccess.into()
}

/// isResClassIII takes a hexagon ID and determines if it is in a
/// Class III resolution (rotated versus the icosahedron and subject
/// to shape distortion adding extra points on icosahedron edges, making
/// them not true hexagons).
///
/// @param h The H3Index to check.
/// @return Returns 1 if the hexagon is class III, otherwise 0.
#[no_mangle]
pub extern "C" fn isResClassIII(h: H3Index) -> c_int {
    CellIndex::try_from(h)
        .map(CellIndex::resolution)
        .map(Resolution::is_class3)
        .unwrap_or_default()
        .into()
}

/// pentagonCount returns the number of pentagons (same at any resolution)
///
/// @return int count of pentagon indexes
#[no_mangle]
pub extern "C" fn pentagonCount() -> c_int {
    Resolution::pentagon_count().into()
}

/// res0CellCount returns the number of resolution 0 cells
///
/// @return int count of resolution 0 cells
#[no_mangle]
pub extern "C" fn res0CellCount() -> c_int {
    BaseCell::count().into()
}
