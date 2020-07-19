use std::convert::From;
use std::error;
use std::fmt;

use spglib_sys as ffi;

/// Possible error codes.
#[derive(Clone, Debug)]
pub enum SpglibError {
    SpacegroupSearchFailed,
    CellStandardizationFailed,
    SymmetryOperationSearchFailed,
    AtomsTooClose,
    PointgroupNotFound,
    NiggliFailed,
    DelaunayFailed,
    ArraySizeShortage,
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
