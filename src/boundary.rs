use crate::LatLng;
use std::ffi::c_int;

/// Maximum number of cell boundary vertices; worst case is pentagon:
/// 5 original verts + 5 edge crossings.
pub const MAX_CELL_BNDRY_VERTS: usize = 10;

/// Cell boundary in latitude/longitude.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CellBoundary {
    /// Number of vertices.
    pub numVerts: c_int,
    /// Vertices in ccw order.
    pub verts: [LatLng; MAX_CELL_BNDRY_VERTS],
}

impl From<h3o::Boundary> for CellBoundary {
    fn from(value: h3o::Boundary) -> Self {
        assert!(value.len() <= MAX_CELL_BNDRY_VERTS);

        let mut boundary = Self {
            numVerts: c_int::try_from(value.len()).expect("too many vertex"),
            ..Default::default()
        };

        for (i, &vertex) in value.iter().enumerate() {
            boundary.verts[i] = vertex.into();
        }

        boundary
    }
}
