//! An h3o wrapper that expose a C API compatible with h3.

// Lints {{{

#![deny(
    nonstandard_style,
    rust_2018_idioms,
    rust_2021_compatibility,
    future_incompatible,
    rustdoc::all,
    rustdoc::missing_crate_level_docs,
    unused,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    clippy::allow_attributes_without_reason,
    clippy::as_underscore,
    clippy::branches_sharing_code,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::default_union_representation,
    clippy::derive_partial_eq_without_eq,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::exhaustive_enums,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::future_not_send,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::missing_const_for_fn,
    clippy::mixed_read_write_in_expression,
    clippy::multiple_inherent_impl,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_collect,
    clippy::non_send_fields_in_send_ty,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::panic,
    clippy::path_buf_push_overwrite,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_pub_crate,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::significant_drop_in_scrutinee,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trailing_empty_array,
    clippy::trait_duplication_in_bounds,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::type_repetition_in_bounds,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unused_peekable,
    clippy::unused_rounding,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads
)]
#![allow(
    // Compat with H3 API.
    non_snake_case,
    // I simply copy/paste H3 comments.
    clippy::doc_markdown,
    // The 90’s called and wanted their charset back.
    clippy::non_ascii_literal,
    // "It requires the user to type the module name twice."
    // => not true here since internal modules are hidden from the users.
    clippy::module_name_repetitions,
    // Usually yes, but not really applicable for most literals in this crate.
    clippy::unreadable_literal,
    // Too many irrelevant warning (about internal invariants).
    clippy::missing_panics_doc,
)]

// }}}

use h3o::{CellIndex, DirectedEdgeIndex, VertexIndex};
use std::ffi::{c_char, CStr};

mod boundary;
mod cell;
mod compact;
mod convert;
mod directed_edge;
mod error;
mod geom;
mod grid;
mod latlng;
mod localij;
mod resolution;
mod vertex;

// TODO: find why cbindgen can't generate #define for those...
// pub const H3O_VERSION_MAJOR: u8 = h3o::VERSION_MAJOR;
// pub const H3O_VERSION_MINOR: u8 = h3o::VERSION_MINOR;
// pub const H3O_VERSION_PATCH: u8 = h3o::VERSION_PATCH;

pub const H3O_VERSION_MAJOR: u8 = 0;
pub const H3O_VERSION_MINOR: u8 = 3;
pub const H3O_VERSION_PATCH: u8 = 0;

pub use boundary::{CellBoundary, MAX_CELL_BNDRY_VERTS};
pub use cell::{
    cellAreaKm2, cellAreaM2, cellAreaRads2, cellToBoundary, cellToCenterChild,
    cellToChildPos, cellToChildren, cellToChildrenSize, cellToLatLng,
    cellToParent, childPosToCell, getBaseCellNumber, getIcosahedronFaces,
    getResolution, isPentagon, isValidCell, maxFaceCount,
};
pub use compact::{compactCells, uncompactCells, uncompactCellsSize};
pub use directed_edge::{
    areNeighborCells, cellsToDirectedEdge, directedEdgeToBoundary,
    directedEdgeToCells, edgeLengthKm, edgeLengthM, edgeLengthRads,
    getDirectedEdgeDestination, getDirectedEdgeOrigin, isValidDirectedEdge,
    originToDirectedEdges,
};
pub use error::{H3Error, H3ErrorCodes};
pub use geom::{
    cellsToLinkedMultiPolygon, destroyLinkedMultiPolygon,
    maxPolygonToCellsSize, polygonToCells, GeoLoop, GeoMultiPolygon,
    GeoPolygon, LinkedGeoLoop, LinkedGeoPolygon, LinkedLatLng,
};
pub use grid::{
    gridDisk, gridDiskDistances, gridDiskDistancesSafe,
    gridDiskDistancesUnsafe, gridDiskUnsafe, gridDisksUnsafe, gridDistance,
    gridPathCells, gridPathCellsSize, gridRingUnsafe, maxGridDiskSize,
};
pub use latlng::{
    greatCircleDistanceKm, greatCircleDistanceM, greatCircleDistanceRads,
    latLngToCell, LatLng,
};
pub use localij::{cellToLocalIj, localIjToCell, CoordIJ};
pub use resolution::{
    getHexagonAreaAvgKm2, getHexagonAreaAvgM2, getHexagonEdgeLengthAvgKm,
    getHexagonEdgeLengthAvgM, getNumCells, getPentagons, getRes0Cells,
    isResClassIII, pentagonCount, res0CellCount,
};
pub use vertex::{cellToVertex, cellToVertexes, isValidVertex, vertexToLatLng};

// -----------------------------------------------------------------------------

/// Identifier for an object (cell, edge, etc) in the H3 system.
///
/// The H3Index fits within a 64-bit unsigned integer.
pub type H3Index = u64;

/// Invalid index used to indicate an error from latLngToCell and related
/// functions or missing data in arrays of H3 indices. Analogous to NaN in
/// floating point.
pub const H3_NULL: H3Index = 0;

// -----------------------------------------------------------------------------

/// Convert from decimal degrees to radians.
///
/// @param degrees The decimal degrees.
/// @return The corresponding radians.
#[no_mangle]
pub extern "C" fn degsToRads(degrees: f64) -> f64 {
    degrees.to_radians()
}

/// Converts an H3 index into a string representation.
///
/// @param h The H3 index to convert.
/// @param str The string representation of the H3 index.
/// @param sz Size of the buffer `str`
/// # Safety
///
/// `s` must points to an array of at least `sz` elements.
#[no_mangle]
pub unsafe extern "C" fn h3ToString(
    h: H3Index,
    s: *mut c_char,
    sz: usize,
) -> H3Error {
    // An unsigned 64 bit integer will be expressed in at most
    // 16 digits plus 1 for the null terminator.
    if sz < 17 {
        return H3ErrorCodes::EFailed.into();
    }

    let string = format!("{h:x}").into_bytes();
    let slice = std::slice::from_raw_parts_mut(s, sz);
    slice[string.len()] = 0;
    for (i, ascii) in string.into_iter().enumerate() {
        slice[i] = ascii as c_char;
    }
    H3ErrorCodes::ESuccess.into()
}

/// Convert from radians to decimal degrees.
///
/// @param radians The radians.
/// @return The corresponding decimal degrees.
#[no_mangle]
pub extern "C" fn radsToDegs(radians: f64) -> f64 {
    radians.to_degrees()
}

/// Converts a string representation of an H3 index into an H3 index.
///
/// @param str The string representation of an H3 index.
/// @return The H3 index corresponding to the string argument, or H3_NULL if
/// invalid.
#[no_mangle]
pub extern "C" fn stringToH3(
    str: *const c_char,
    out: Option<&mut H3Index>,
) -> H3Error {
    fn inner(str: *const c_char) -> Result<H3Index, H3Error> {
        // SAFETY: `str` must point to a null-terminated string.
        // See CStr::from_ptr documentation for more info.
        unsafe {
            let s = CStr::from_ptr(str)
                .to_str()
                .map_err(|_| H3Error::from(H3ErrorCodes::EFailed))?;

            s.parse::<CellIndex>()
                .map(Into::into)
                .or_else(|_| s.parse::<DirectedEdgeIndex>().map(Into::into))
                .or_else(|_| s.parse::<VertexIndex>().map(Into::into))
                .map_err(|_| H3ErrorCodes::EFailed.into())
        }
    }

    delegate_inner!(inner(str), out)
}

/// Call the provided inner function, set the out pointer to result on success
/// and propagate errors.
#[macro_export]
#[doc(hidden)]
macro_rules! delegate_inner {
    ($inner:expr, $out:ident) => {
        match $inner {
            Ok(res) => {
                *$out.expect("null pointer") = res;
                H3ErrorCodes::ESuccess.into()
            }
            Err(err) => err,
        }
    };
}
