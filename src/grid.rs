use crate::{convert, delegate_inner, H3Error, H3ErrorCodes, H3Index, H3_NULL};
use h3o::{error::LocalIjError, CellIndex};
use std::ffi::c_int;

/// Produce cells within grid distance k of the origin cell.
///
/// k-ring 0 is defined as the origin cell, k-ring 1 is defined as k-ring 0 and
/// all neighboring cells, and so on.
///
/// Output is placed in the provided array in no particular order. Elements of
/// the output array may be left zero, as can happen when crossing a pentagon.
///
/// @param  origin   origin cell
/// @param  k        k >= 0
/// @param  out      zero-filled array which must be of size maxGridDiskSize(k)
///
/// # Safety
///
/// `out` must points to an array of at least `maxGridDiskSize(k)` elements.
#[no_mangle]
pub unsafe extern "C" fn gridDisk(
    origin: H3Index,
    k: c_int,
    out: *mut H3Index,
) -> H3Error {
    fn inner(
        origin: H3Index,
        k: u32,
        out: &mut [H3Index],
    ) -> Result<(), H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let mut count = 0;

        // Try fast version first.
        for result in origin.grid_disk_fast(k) {
            if let Some(index) = result {
                out[count] = index.into();
                count += 1;
            } else {
                out[..count].fill(H3_NULL);
                count = 0;
                break;
            }
        }

        // Fast version failed, fallback on the slowert (but safer) approach.
        if count == 0 {
            for index in origin.grid_disk_safe(k) {
                out[count] = index.into();
                count += 1;
            }
        }

        Ok(())
    }

    // Get the expected size of the output variables.
    let Ok(k) = u32::try_from(k) else { return H3ErrorCodes::EDomain.into() };
    let size = h3o::max_grid_disk_size(k);

    // Convert pointers to slices.
    // This is the part that goes UB if the caller didn't respect the contract.
    let len = usize::try_from(size).expect("overflow");
    let slice = std::slice::from_raw_parts_mut(out, len);

    if let Err(err) = inner(origin, k, slice) {
        return err;
    }

    H3ErrorCodes::ESuccess.into()
}

/// Produce cells and their distances from the given origin cell, up to
/// distance k.
///
/// k-ring 0 is defined as the origin cell, k-ring 1 is defined as k-ring 0 and
/// all neighboring cells, and so on.
///
/// Output is placed in the provided array in no particular order. Elements of
/// the output array may be left zero, as can happen when crossing a pentagon.
///
/// @param  origin      origin cell
/// @param  k           k >= 0
/// @param  out         zero-filled array which must be of size
/// maxGridDiskSize(k)
/// @param  distances   NULL or a zero-filled array which must be of size
///                     maxGridDiskSize(k)
/// # Safety
///
/// `out` and `distances` must points to an array of at least
/// `maxGridDiskSize(k)` elements each.
#[no_mangle]
pub unsafe extern "C" fn gridDiskDistances(
    origin: H3Index,
    k: c_int,
    out: *mut H3Index,
    distances: *mut c_int,
) -> H3Error {
    fn inner(
        origin: H3Index,
        k: u32,
        cells: &mut [H3Index],
        dists: &mut [c_int],
    ) -> Result<(), H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let mut count = 0;

        // Try fast version first.
        for result in origin.grid_disk_distances_fast(k) {
            if let Some((index, dist)) = result {
                cells[count] = index.into();
                dists[count] = dist.try_into().expect("distance overflow");
                count += 1;
            } else {
                cells[..count].fill(H3_NULL);
                dists[..count].fill(0);
                count = 0;
                break;
            }
        }

        // Fast version failed, fallback on the slowert (but safer) approach.
        if count == 0 {
            for (index, dist) in origin.grid_disk_distances_safe(k) {
                cells[count] = index.into();
                dists[count] = dist.try_into().expect("distance overflow");
                count += 1;
            }
        }

        Ok(())
    }

    // Get the expected size of the output variables.
    let Ok(k) = u32::try_from(k) else { return H3ErrorCodes::EDomain.into() };
    let size = h3o::max_grid_disk_size(k);

    // Convert pointers to slices.
    // This is the part that goes UB if the caller didn't respect the contract.
    let len = usize::try_from(size).expect("overflow");
    let cells = std::slice::from_raw_parts_mut(out, len);
    let dists = std::slice::from_raw_parts_mut(distances, len);

    if let Err(err) = inner(origin, k, cells, dists) {
        return err;
    }

    H3ErrorCodes::ESuccess.into()
}

/// Safe but slow version of gridDiskDistances (also called by it when needed).
///
/// Adds the origin cell to the output set (treating it as a hash set)
/// and recurses to its neighbors, if needed.
///
/// @param  origin      Origin cell
/// @param  k           Maximum distance to move from the origin
/// @param  out         Array treated as a hash set, elements being either
///                     H3Index or 0.
/// @param  distances   Scratch area, with elements paralleling the out array.
///                     Elements indicate ijk distance from the origin cell to
///                     the output cell
///
/// # Safety
///
/// `out` and `distances` must points to an array of at least
/// `maxGridDiskSize(k)` elements each.
#[no_mangle]
pub unsafe extern "C" fn gridDiskDistancesSafe(
    origin: H3Index,
    k: c_int,
    out: *mut H3Index,
    distances: *mut c_int,
) -> H3Error {
    fn inner(
        origin: H3Index,
        k: c_int,
    ) -> Result<(u64, impl Iterator<Item = (CellIndex, u32)>), H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let k = u32::try_from(k).map_err(|_| H3ErrorCodes::EDomain)?;
        Ok((
            h3o::max_grid_disk_size(k),
            origin.grid_disk_distances_safe(k),
        ))
    }

    match inner(origin, k) {
        Ok((len, iter)) => {
            let len = usize::try_from(len).expect("overflow");
            let cells = std::slice::from_raw_parts_mut(out, len);
            let dists = std::slice::from_raw_parts_mut(distances, len);
            for (i, (cell_index, dist)) in iter.enumerate() {
                cells[i] = cell_index.into();
                dists[i] = dist.try_into().expect("distance overflow");
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// gridDiskDistancesUnsafe produces indexes within k distance of the origin
/// index. Output behavior is undefined when one of the indexes returned by this
/// function is a pentagon or is in the pentagon distortion area.
///
/// k-ring 0 is defined as the origin index, k-ring 1 is defined as k-ring 0 and
/// all neighboring indexes, and so on.
///
/// Output is placed in the provided array in order of increasing distance from
/// the origin. The distances in hexagons is placed in the distances array at
/// the same offset.
///
/// @param origin Origin location.
/// @param k k >= 0
/// @param out Array which must be of size maxGridDiskSize(k).
/// @param distances Null or array which must be of size maxGridDiskSize(k).
/// @return 0 if no pentagon or pentagonal distortion area was encountered.
///
/// # Safety
///
/// `out` and `distances` must points to an array of at least
/// `maxGridDiskSize(k)` elements each.
#[no_mangle]
pub unsafe extern "C" fn gridDiskDistancesUnsafe(
    origin: H3Index,
    k: c_int,
    out: *mut H3Index,
    distances: *mut c_int,
) -> H3Error {
    fn inner(
        origin: H3Index,
        k: c_int,
    ) -> Result<(u64, impl Iterator<Item = Option<(CellIndex, u32)>>), H3Error>
    {
        let origin = CellIndex::try_from(origin)?;
        let k = u32::try_from(k).map_err(|_| H3ErrorCodes::EDomain)?;
        Ok((
            h3o::max_grid_disk_size(k),
            origin.grid_disk_distances_fast(k),
        ))
    }

    match inner(origin, k) {
        Ok((len, iter)) => {
            let len = usize::try_from(len).expect("overflow");
            let cells = std::slice::from_raw_parts_mut(out, len);
            let dists = std::slice::from_raw_parts_mut(distances, len);
            for (i, item) in iter.enumerate() {
                if let Some((cell_index, dist)) = item {
                    cells[i] = cell_index.into();
                    dists[i] = dist.try_into().expect("distance overflow");
                } else {
                    return H3ErrorCodes::EPentagon.into();
                }
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// gridDiskUnsafe produces indexes within k distance of the origin index.
/// Output behavior is undefined when one of the indexes returned by this
/// function is a pentagon or is in the pentagon distortion area.
///
/// k-ring 0 is defined as the origin index, k-ring 1 is defined as k-ring 0 and
/// all neighboring indexes, and so on.
///
/// Output is placed in the provided array in order of increasing distance from
/// the origin.
///
/// @param origin Origin location.
/// @param k k >= 0
/// @param out Array which must be of size maxGridDiskSize(k).
/// @return 0 if no pentagon or pentagonal distortion area was encountered.
///
/// # Safety
///
/// `out` must points to an array of at least `maxGridDiskSize(k)` elements.
#[no_mangle]
pub unsafe extern "C" fn gridDiskUnsafe(
    origin: H3Index,
    k: c_int,
    out: *mut H3Index,
) -> H3Error {
    fn inner(
        origin: H3Index,
        k: c_int,
    ) -> Result<(u64, impl Iterator<Item = Option<CellIndex>>), H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let k = u32::try_from(k).map_err(|_| H3ErrorCodes::EDomain)?;
        Ok((h3o::max_grid_disk_size(k), origin.grid_disk_fast(k)))
    }

    match inner(origin, k) {
        Ok((len, iter)) => {
            let len = usize::try_from(len).expect("overflow");
            let slice = std::slice::from_raw_parts_mut(out, len);
            for (i, item) in iter.enumerate() {
                if let Some(cell_index) = item {
                    slice[i] = cell_index.into();
                } else {
                    return H3ErrorCodes::EPentagon.into();
                }
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// gridDisksUnsafe takes an array of input hex IDs and a max k-ring and returns
/// an array of hexagon IDs sorted first by the original hex IDs and then by the
/// k-ring (0 to max), with no guaranteed sorting within each k-ring group.
///
/// @param h3Set A pointer to an array of H3Indexes
/// @param length The total number of H3Indexes in h3Set
/// @param k The number of rings to generate
/// @param out A pointer to the output memory to dump the new set of H3Indexes to
///            The memory block should be equal to maxGridDiskSize(k) * length
/// @return 0 if no pentagon is encountered. Cannot trust output otherwise
///
/// # Safety
///
/// - `h3Set` must points to an array of at least `length` elements.
/// - `out` must points to an array of at least `length * maxGridDiskSize(k)`
///   elements.
#[no_mangle]
pub unsafe extern "C" fn gridDisksUnsafe(
    h3Set: *mut H3Index,
    length: c_int,
    k: c_int,
    out: *mut H3Index,
) -> H3Error {
    unsafe fn inner(
        h3Set: *mut H3Index,
        length: c_int,
        k: c_int,
    ) -> Result<(u64, impl Iterator<Item = Option<CellIndex>>), H3Error> {
        let indexes = convert::h3ptr_to_h3oslice_mut(h3Set, length)?;
        let k = u32::try_from(k).map_err(|_| H3ErrorCodes::EDomain)?;
        let count = u64::try_from(indexes.len()).expect("index count overflow");
        Ok((
            h3o::max_grid_disk_size(k) * count,
            CellIndex::grid_disks_fast(indexes.iter().copied(), k),
        ))
    }

    if length == 0 {
        return H3ErrorCodes::ESuccess.into();
    }

    match inner(h3Set, length, k) {
        Ok((len, iter)) => {
            let len = usize::try_from(len).expect("overflow");
            let slice = std::slice::from_raw_parts_mut(out, len);
            for (i, item) in iter.enumerate() {
                if let Some(cell_index) = item {
                    slice[i] = cell_index.into();
                } else {
                    return H3ErrorCodes::EPentagon.into();
                }
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// Produces the grid distance between the two indexes.
///
/// This function may fail to find the distance between two indexes, for
/// example if they are very far apart. It may also fail when finding
/// distances for indexes on opposite sides of a pentagon.
///
/// @param origin Index to find the distance from.
/// @param index Index to find the distance to.
/// @return The distance, or a negative number if the library could not
/// compute the distance.
#[no_mangle]
pub extern "C" fn gridDistance(
    origin: H3Index,
    h3: H3Index,
    distance: Option<&mut i64>,
) -> H3Error {
    fn inner(origin: H3Index, h3: H3Index) -> Result<i64, H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let h3 = CellIndex::try_from(h3)?;

        Ok(origin.grid_distance(h3)?.into())
    }

    delegate_inner!(inner(origin, h3), distance)
}

/// Given two H3 indexes, return the line of indexes between them (inclusive).
///
/// This function may fail to find the line between two indexes, for
/// example if they are very far apart. It may also fail when finding
/// distances for indexes on opposite sides of a pentagon.
///
/// Notes:
///
///  - The specific output of this function should not be considered stable
///    across library versions. The only guarantees the library provides are
///    that the line length will be `gridDistance(start, end) + 1` and that
///    every index in the line will be a neighbor of the preceding index.
///  - Lines are drawn in grid space, and may not correspond exactly to either
///    Cartesian lines or great arcs.
///
/// @param start Start index of the line
/// @param end End index of the line
/// @param out Output array
/// @return 0 on success, or another value on failure.
///
/// # Safety
///
/// `out` must points to an array of at least `gridPathCellsSize(start, end)` elements.
#[no_mangle]
pub unsafe extern "C" fn gridPathCells(
    start: H3Index,
    end: H3Index,
    out: *mut H3Index,
) -> H3Error {
    fn inner(
        start: H3Index,
        end: H3Index,
    ) -> Result<
        (i64, impl Iterator<Item = Result<CellIndex, LocalIjError>>),
        H3Error,
    > {
        let start = CellIndex::try_from(start)?;
        let end = CellIndex::try_from(end)?;

        Ok((
            start.grid_path_cells_size(end)?.into(),
            start.grid_path_cells(end)?,
        ))
    }

    match inner(start, end) {
        Ok((len, iter)) => {
            let len = usize::try_from(len).expect("overflow");
            let slice = std::slice::from_raw_parts_mut(out, len);
            for (i, item) in iter.enumerate() {
                match item {
                    Ok(cell_index) => slice[i] = cell_index.into(),
                    Err(err) => return err.into(),
                }
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// Number of indexes in a line from the start index to the end index,
/// to be used for allocating memory. Returns a negative number if the
/// line cannot be computed.
///
/// @param start Start index of the line
/// @param end End index of the line
/// @param size Size of the line
/// @returns 0 on success, or another value on error
#[no_mangle]
pub extern "C" fn gridPathCellsSize(
    start: H3Index,
    end: H3Index,
    size: Option<&mut i64>,
) -> H3Error {
    fn inner(start: H3Index, end: H3Index) -> Result<i64, H3Error> {
        let start = CellIndex::try_from(start)?;
        let end = CellIndex::try_from(end)?;

        Ok(start.grid_path_cells_size(end)?.into())
    }

    delegate_inner!(inner(start, end), size)
}

/// Returns the "hollow" ring of hexagons at exactly grid distance k from
/// the origin hexagon. In particular, k=0 returns just the origin hexagon.
///
/// A nonzero failure code may be returned in some cases, for example,
/// if a pentagon is encountered.
/// Failure cases may be fixed in future versions.
///
/// @param origin Origin location.
/// @param k k >= 0
/// @param out Array which must be of size 6 * k (or 1 if k == 0)
/// @return 0 if successful; nonzero otherwise.
///
/// # Safety
///
/// `out` must points to an array of at least `6 * k` (or `1` if `k == 0`)
/// elements.
#[no_mangle]
pub unsafe extern "C" fn gridRingUnsafe(
    origin: H3Index,
    k: c_int,
    out: *mut H3Index,
) -> H3Error {
    fn inner(
        origin: H3Index,
        k: c_int,
    ) -> Result<(u32, impl Iterator<Item = Option<CellIndex>>), H3Error> {
        let origin = CellIndex::try_from(origin)?;
        let k = u32::try_from(k).map_err(|_| H3ErrorCodes::EDomain)?;
        let len = if k == 0 { 1 } else { 6 * k };
        Ok((len, origin.grid_ring_fast(k)))
    }

    match inner(origin, k) {
        Ok((len, iter)) => {
            let len = usize::try_from(len).expect("overflow");
            let slice = std::slice::from_raw_parts_mut(out, len);
            for (i, item) in iter.enumerate() {
                if let Some(cell_index) = item {
                    slice[i] = cell_index.into();
                } else {
                    return H3ErrorCodes::EPentagon.into();
                }
            }
            H3ErrorCodes::ESuccess.into()
        }
        Err(err) => err,
    }
}

/// Maximum number of cells that result from the gridDisk algorithm with the
/// given k. Formula source and proof: `<https://oeis.org/A003215>`
///
/// @param   k   k value, k >= 0.
/// @param out   size in indexes
#[no_mangle]
pub extern "C" fn maxGridDiskSize(k: c_int, out: Option<&mut i64>) -> H3Error {
    fn inner(k: c_int) -> Result<i64, H3Error> {
        Ok(u32::try_from(k)
            .map_err(|_| H3ErrorCodes::EDomain)
            .map(h3o::max_grid_disk_size)?
            .try_into()
            .expect("grid disk size overflow"))
    }

    delegate_inner!(inner(k), out)
}
