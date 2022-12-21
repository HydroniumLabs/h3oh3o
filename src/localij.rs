use crate::{delegate_inner, H3Error, H3ErrorCodes, H3Index};
use h3o::CellIndex;
use std::ffi::c_int;

/// IJ hexagon coordinates.
///
/// Each axis is spaced 120 degrees apart.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CoordIJ {
    /// i component.
    pub i: c_int,
    /// j component.
    pub j: c_int,
}

/// Produces ij coordinates for an index anchored by an origin.
///
/// The coordinate space used by this function may have deleted
/// regions or warping due to pentagonal distortion.
///
/// Coordinates are only comparable if they come from the same
/// origin index.
///
/// Failure may occur if the index is too far away from the origin
/// or if the index is on the other side of a pentagon.
///
/// This function's output is not guaranteed
/// to be compatible across different versions of H3.
///
/// @param origin An anchoring index for the ij coordinate system.
/// @param index Index to find the coordinates of
/// @param mode Mode, must be 0
/// @param out ij coordinates of the index will be placed here on success
/// @return 0 on success, or another value on failure.
#[no_mangle]
pub extern "C" fn cellToLocalIj(
    origin: H3Index,
    h3: H3Index,
    mode: u32,
    out: Option<&mut CoordIJ>,
) -> H3Error {
    fn inner(
        origin: H3Index,
        h3: H3Index,
        mode: u32,
    ) -> Result<CoordIJ, H3Error> {
        if mode != 0 {
            return Err(H3ErrorCodes::EDomain.into());
        }
        let origin = CellIndex::try_from(origin)?;
        let h3 = CellIndex::try_from(h3)?;
        let localij = h3.to_local_ij(origin)?;
        Ok(CoordIJ {
            i: localij.i(),
            j: localij.j(),
        })
    }

    delegate_inner!(inner(origin, h3, mode), out)
}

/// Produces an index for ij coordinates anchored by an origin.
///
/// The coordinate space used by this function may have deleted
/// regions or warping due to pentagonal distortion.
///
/// Failure may occur if the index is too far away from the origin
/// or if the index is on the other side of a pentagon.
///
/// This function's output is not guaranteed
/// to be compatible across different versions of H3.
///
/// @param origin An anchoring index for the ij coordinate system.
/// @param out ij coordinates to index.
/// @param mode Mode, must be 0
/// @param index Index will be placed here on success.
/// @return 0 on success, or another value on failure.
#[no_mangle]
pub extern "C" fn localIjToCell(
    origin: H3Index,
    ij: Option<&CoordIJ>,
    mode: u32,
    out: Option<&mut H3Index>,
) -> H3Error {
    fn inner(
        origin: H3Index,
        ij: CoordIJ,
        mode: u32,
    ) -> Result<H3Index, H3Error> {
        if mode != 0 {
            return Err(H3ErrorCodes::EDomain.into());
        }
        let origin = CellIndex::try_from(origin)?;
        let localij = h3o::LocalIJ::new_unchecked(origin, ij.i, ij.j);
        Ok(CellIndex::try_from(localij)?.into())
    }

    delegate_inner!(inner(origin, *ij.expect("null pointer"), mode), out)
}
