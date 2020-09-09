//! Data structures to contain physical crystal systems.

use std::os::raw::c_int;

use spglib_sys as ffi;

use crate::error::SpglibError;

/// A crystalline cell.
#[derive(Clone, Debug)]
pub struct Cell {
    /// Lattice vectors.
    pub lattice: [[f64; 3]; 3],
    /// Position of each atom.
    pub positions: Vec<[f64; 3]>,
    /// Type of each atom.
    pub types: Vec<i32>,
}

impl Cell {
    /// Returns a new crystalline cell.
    pub fn new(lattice: &[[f64; 3]; 3], positions: &[[f64; 3]], types: &[i32]) -> Cell {
        let lattice = *lattice;
        let positions = Vec::from(positions);
        let types = Vec::from(types);
        Cell {
            lattice,
            positions,
            types,
        }
    }

    /// Standardizes the cell with a symmetry search.
    /// Refer to the full documentation of the implementation [here](https://spglib.github.io/spglib/api.html#spg-standardize-cell).
    /// 
    /// # Example
    /// 
    /// ```
    /// use spglib::cell::Cell;
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
    /// cell.standardize(true, false, 1.0e-6).unwrap();
    /// assert_eq!(
    ///     cell.lattice, 
    ///     [
    ///         [-2.0, 2.0, 2.0],
    ///         [2.0, -2.0, 2.0],
    ///         [2.0, 2.0, -2.0]
    ///     ]
    /// );
    /// ```
    pub fn standardize(&mut self, to_primitive: bool, no_idealize: bool, symprec: f64) -> Result<(), SpglibError> {
        let to_primitive = if to_primitive {1} else {0};
        let no_idealize = if no_idealize {1} else {0};
        let res = unsafe {
            ffi::spg_standardize_cell(
                self.lattice.as_ptr() as *mut [f64; 3],
                self.positions.as_ptr() as *mut [f64; 3],
                self.types.as_ptr() as *mut c_int,
                self.positions.len() as c_int,
                to_primitive,
                no_idealize,
                symprec,
            )
        };
        if res == 0 {
            return Err(SpglibError::CellStandardizationFailed);
        }
        Ok(())
    }

    /// Applies a Delaunay reduction to the cell.
    /// Refer to the full documentation of the implementation [here](https://spglib.github.io/spglib/api.html#spg-delaunay-reduce).
    pub fn delaunay_reduce(&mut self, eps: f64) -> Result<(), SpglibError> {
        unimplemented!()
    }

    /// Applies a Niggli reduction to the cell.
    /// Refer to the full documentation of the implementation [here](https://spglib.github.io/spglib/api.html#spg-niggli-reduce).
    pub fn niggli_reduce(&mut self, eps: f64) -> Result<(), SpglibError> {
        unimplemented!()
    }

    pub fn ir_reciprocal_mesh(&self, mesh: (i32, i32, i32), shift: (bool, bool, bool)) -> Result<(), SpglibError> {
        unimplemented!()
    }
}
