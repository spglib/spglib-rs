//! Data structures to contain physical crystal systems.

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
}
