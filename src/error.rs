//! Error types.

use std::convert::From;
use std::error;
use std::fmt;

use spglib_sys as ffi;

/// Possible error codes.
#[derive(Clone, Debug)]
pub enum SpglibError {
    /// Raised when spacegroup search fails.
    SpacegroupSearchFailed,
    /// Raised when cell standardization routine fails.
    CellStandardizationFailed,
    /// Raised when symmetry operation search fails.
    SymmetryOperationSearchFailed,
    /// Raised when atoms in a cell occupy the same site.
    AtomsTooClose,
    /// Raised when pointgroup search fails.
    PointgroupNotFound,
    /// Raised when Niggli reduction routine fails.
    NiggliFailed,
    /// Raised when Delaunay reduction routine fails.
    DelaunayFailed,
    /// Raised when an array argument has insufficient capacity.
    ArraySizeShortage,
    /// Raised for any unknown errors.
    Unknown,
}

impl From<ffi::SpglibError> for SpglibError {
    fn from(item: ffi::SpglibError) -> Self {
        match item {
            1 => SpglibError::SpacegroupSearchFailed,
            2 => SpglibError::CellStandardizationFailed,
            3 => SpglibError::SymmetryOperationSearchFailed,
            4 => SpglibError::AtomsTooClose,
            5 => SpglibError::PointgroupNotFound,
            6 => SpglibError::NiggliFailed,
            7 => SpglibError::DelaunayFailed,
            8 => SpglibError::ArraySizeShortage,
            _ => SpglibError::Unknown,
        }
    }
}

impl fmt::Display for SpglibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpglibError::SpacegroupSearchFailed => write!(f, "space group search failed"),
            SpglibError::CellStandardizationFailed => write!(f, "cell standardization failed"),
            SpglibError::SymmetryOperationSearchFailed => {
                write!(f, "symmetry operation search failed")
            }
            SpglibError::AtomsTooClose => write!(f, "atoms too close"),
            SpglibError::PointgroupNotFound => write!(f, "pointgroup not found"),
            SpglibError::NiggliFailed => write!(f, "niggli failed"),
            SpglibError::DelaunayFailed => write!(f, "delaunay failed"),
            SpglibError::ArraySizeShortage => write!(f, "array size shortage"),
            SpglibError::Unknown => write!(f, "unknown error"),
        }
    }
}

impl error::Error for SpglibError {}
