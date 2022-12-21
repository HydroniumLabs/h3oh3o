/// Result code (success or specific error) from an H3 operation.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct H3Error(u32);

impl From<H3ErrorCodes> for H3Error {
    fn from(value: H3ErrorCodes) -> Self {
        Self(value as u32)
    }
}

impl From<h3o::error::InvalidVertex> for H3Error {
    fn from(_: h3o::error::InvalidVertex) -> Self {
        H3ErrorCodes::EDomain.into()
    }
}

impl From<h3o::error::InvalidLatLng> for H3Error {
    fn from(_: h3o::error::InvalidLatLng) -> Self {
        H3ErrorCodes::ELatlngDomain.into()
    }
}

impl From<h3o::error::InvalidResolution> for H3Error {
    fn from(_: h3o::error::InvalidResolution) -> Self {
        H3ErrorCodes::EResDomain.into()
    }
}

impl From<h3o::error::InvalidCellIndex> for H3Error {
    fn from(_: h3o::error::InvalidCellIndex) -> Self {
        H3ErrorCodes::ECellInvalid.into()
    }
}

impl From<h3o::error::InvalidDirectedEdgeIndex> for H3Error {
    fn from(_: h3o::error::InvalidDirectedEdgeIndex) -> Self {
        H3ErrorCodes::EDirEdgeInvalid.into()
    }
}

impl From<h3o::error::InvalidVertexIndex> for H3Error {
    fn from(_: h3o::error::InvalidVertexIndex) -> Self {
        H3ErrorCodes::EVertexInvalid.into()
    }
}

impl From<h3o::error::ResolutionMismatch> for H3Error {
    fn from(_: h3o::error::ResolutionMismatch) -> Self {
        H3ErrorCodes::EResMismatch.into()
    }
}

impl From<h3o::error::CompactionError> for H3Error {
    fn from(value: h3o::error::CompactionError) -> Self {
        match value {
            h3o::error::CompactionError::HeterogeneousResolution => {
                H3ErrorCodes::EResMismatch
            }
            h3o::error::CompactionError::DuplicateInput => {
                H3ErrorCodes::EDuplicateInput
            }
            _ => H3ErrorCodes::EFailed,
        }
        .into()
    }
}

impl From<h3o::error::OutlinerError> for H3Error {
    fn from(value: h3o::error::OutlinerError) -> Self {
        match value {
            h3o::error::OutlinerError::HeterogeneousResolution => {
                H3ErrorCodes::EResMismatch
            }
            h3o::error::OutlinerError::DuplicateInput => {
                H3ErrorCodes::EDuplicateInput
            }
            _ => H3ErrorCodes::EFailed,
        }
        .into()
    }
}

impl From<h3o::error::InvalidGeometry> for H3Error {
    fn from(_: h3o::error::InvalidGeometry) -> Self {
        H3ErrorCodes::EFailed.into()
    }
}

impl From<h3o::error::LocalIjError> for H3Error {
    fn from(value: h3o::error::LocalIjError) -> Self {
        match value {
            h3o::error::LocalIjError::ResolutionMismatch => {
                H3ErrorCodes::EResMismatch
            }
            h3o::error::LocalIjError::Pentagon => H3ErrorCodes::EPentagon,
            h3o::error::LocalIjError::HexGrid(_) => H3ErrorCodes::EFailed,
            _ => H3ErrorCodes::EFailed,
        }
        .into()
    }
}

/// cbindgen:rename-all=ScreamingSnakeCase
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum H3ErrorCodes {
    /// Success (no error).
    ESuccess = 0,
    /// The operation failed but a more specific error is not available.
    EFailed = 1,
    /// Argument was outside of acceptable range (when a more specific error
    /// code is not available).
    EDomain = 2,
    /// Latitude or longitude arguments were outside of acceptable range.
    ELatlngDomain = 3,
    /// Resolution argument was outside of acceptable range.
    EResDomain = 4,
    /// `H3Index` cell argument was not valid.
    ECellInvalid = 5,
    /// `H3Index` directed edge argument was not valid.
    EDirEdgeInvalid = 6,
    /// `H3Index` undirected edge argument was not valid.
    EUndirEdgeInvalid = 7,
    /// `H3Index` vertex argument was not valid.
    EVertexInvalid = 8,
    /// Pentagon distortion was encountered which the algorithm could not handle
    /// it.
    EPentagon = 9,
    /// Duplicate input was encountered in the arguments and the algorithm could
    /// not handle it.
    EDuplicateInput = 10,
    /// `H3Index` cell arguments were not neighbors.
    ENotNeighbors = 11,
    /// `H3Index` cell arguments had incompatible resolutions.
    EResMismatch = 12,
    /// Necessary memory allocation failed.
    EMemoryAlloc = 13,
    /// Bounds of provided memory were not large enough.
    EMemoryBounds = 14,
    // Mode or flags argument was not valid.
    EOptionInvalid = 15,
}
