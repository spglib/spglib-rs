//! Rust bindings to spglib.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod cell;
pub mod dataset;
pub mod error;
pub mod spacegroup;

use spglib_sys as ffi;

/// Returns the major version of the underlying spglib C library.
pub fn major_version() -> i32 {
    unsafe { ffi::spg_get_major_version() }
}

/// Returns the minor version of the underlying spglib C library.
pub fn minor_version() -> i32 {
    unsafe { ffi::spg_get_minor_version() }
}

/// Returns the micro version of the underlying spglib C library.
pub fn micro_version() -> i32 {
    unsafe { ffi::spg_get_micro_version() }
}

/// Returns the hall number for a set of symmetry operations.
///
/// # Example
///
/// Get the hall number for a BCC cell in a roundabout way.
///
/// ```
/// use spglib::hall_number_from_symmetry;
/// use spglib::cell::Cell;
/// use spglib::dataset::Dataset;
///
/// let lattice = [
///     [4.0, 0.0, 0.0],
///     [0.0, 4.0, 0.0],
///     [0.0, 0.0, 4.0]
/// ];
/// let positions = [
///     [0.0, 0.0, 0.0],
///     [0.5, 0.5, 0.5]
/// ];
/// let types = [1, 1];
/// let mut cell = Cell::new(&lattice, &positions, &types);
/// let mut dataset = Dataset::new(&mut cell, 1.0e-6);
/// let hall_number = hall_number_from_symmetry(&mut dataset.rotations, &mut dataset.translations, 1.0e-6);
/// assert_eq!(hall_number, dataset.hall_number);
/// ```
pub fn hall_number_from_symmetry(
    rotations: &mut [[[i32; 3]; 3]],
    translations: &mut [[f64; 3]],
    symprec: f64,
) -> i32 {
    unsafe {
        ffi::spg_get_hall_number_from_symmetry(
            rotations.as_ptr() as *mut [[i32; 3]; 3],
            translations.as_ptr() as *mut [f64; 3],
            rotations.len() as i32,
            symprec,
        )
    }
}
