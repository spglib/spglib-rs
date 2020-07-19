//! Crystallographic information.

use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};

use spglib_sys as ffi;

use crate::error::SpglibError;

/// Container for a structure's crystallographic properties.
#[derive(Clone, Debug)]
pub struct Dataset {
    /// The space group type number defined in International Tables for Crystallography.
    pub spacegroup_number: i32,
    /// The serial number from 1-530.
    pub hall_number: i32,
    /// The full Hermann-Mauguin notation.
    pub international_symbol: String,
    /// The hall symbol.
    pub hall_symbol: String,
    /// The information on unique axis, setting, or cell choices.
    pub choice: String,
    /// The result of space-group-type matching under a set of unique axis, setting, and cell choices.
    /// For more detail: https://spglib.github.io/spglib/definition.html#transformation-matrix-boldsymbol-p-and-origin-shift-boldsymbol-p
    pub transformation_matrix: [[f64; 3]; 3],
    /// The result of space-group-type matching under a set of unique axis, setting, and cell choices.
    /// For more detail: https://spglib.github.io/spglib/definition.html#transformation-matrix-boldsymbol-p-and-origin-shift-boldsymbol-p
    pub origin_shift: [f64; 3],
    /// The number of symmetry operations.
    pub n_operations: i32,
    /// The rotation symmetry operations.
    pub rotations: Vec<[[i32; 3]; 3]>,
    /// The translation symmetry operations.
    pub translations: Vec<[f64; 3]>,
    /// The number of atoms in the input cell.
    pub n_atoms: i32,
    /// The wyckoff letters encoded as integer numbers.
    pub wyckoffs: Vec<i32>,
    /// Site symmetry symbols for a given space group type.
    pub site_symmetry_symbols: Vec<String>,
    /// The mapping table from the atomic indices of the input cell to the atomic indices of symmetrically independent atoms.
    pub equivalent_atoms: Vec<i32>,
    /// This is almost equivalent to equivalent_atoms. But symmetry of the primitive cell is used to determine the symmetrically equivalent atoms.
    pub crystallographic_orbits: Vec<i32>,
    /// Non-standardized basis vectors of a primitive cell in the input cell.
    pub primitive_lattice: [[f64; 3]; 3],
    /// The atomic indices in the primitive cell of the input structure.
    pub mapping_to_primitive: Vec<i32>,
    /// Number of atoms in the standardized cell after idealization.
    pub n_std_atoms: i32,
    /// Lattice of the standardized cell after idealization.
    pub std_lattice: [[f64; 3]; 3],
    /// Types in the standardized cell after idealization.
    pub std_types: Vec<i32>,
    /// Positions in the standardized cell after idealization.
    pub std_positions: Vec<[f64; 3]>,
    /// Rotation matrix which rotates the cell structure from the pre idealization state to the post idealized state.
    pub std_rotation_matrix: [[f64; 3]; 3],
    /// The atomic indices in the primitive cell of the standardized crystal structure where the same number represents the same atom in the primitive cell.
    pub std_mapping_to_primitive: Vec<i32>,
    /// The symbol of the crystallographic point group in Hermann-Mauguin notation.
    pub pointgroup_symbol: String,
}

impl TryFrom<*mut ffi::SpglibDataset> for Dataset {
    // TODO
    type Error = SpglibError;

    fn try_from(value: *mut ffi::SpglibDataset) -> Result<Self, Self::Error> {
        // dereference the raw pointer
        let ptr = unsafe { &mut *value };
        // process fields
        let spacegroup_number = ptr.spacegroup_number as i32;
        let hall_number = ptr.hall_number as i32;
        let international_symbol =
            match CString::from(unsafe { CStr::from_ptr(ptr.international_symbol.as_ptr()) })
                .to_str()
            {
                Ok(s) => String::from(s),
                Err(_) => return Err(SpglibError::Unknown),
            };
        let hall_symbol =
            match CString::from(unsafe { CStr::from_ptr(ptr.hall_symbol.as_ptr()) }).to_str() {
                Ok(s) => String::from(s),
                Err(_) => return Err(SpglibError::Unknown),
            };
        let choice = match CString::from(unsafe { CStr::from_ptr(ptr.choice.as_ptr()) }).to_str() {
            Ok(s) => String::from(s),
            Err(_) => return Err(SpglibError::Unknown),
        };
        let transformation_matrix = ptr.transformation_matrix;
        let origin_shift = ptr.origin_shift;
        let n_operations = ptr.n_operations as i32;
        let rotations = unsafe {
            Vec::from_raw_parts(ptr.rotations, n_operations as usize, n_operations as usize)
        };
        let translations = unsafe {
            Vec::from_raw_parts(
                ptr.translations,
                n_operations as usize,
                n_operations as usize,
            )
        };
        let n_atoms = ptr.n_atoms as i32;
        let wyckoffs =
            unsafe { Vec::from_raw_parts(ptr.wyckoffs, n_atoms as usize, n_atoms as usize) };
        // TODO
        let site_symmetry_symbols = Vec::new();
        let equivalent_atoms = unsafe {
            Vec::from_raw_parts(ptr.equivalent_atoms, n_atoms as usize, n_atoms as usize)
        };
        let crystallographic_orbits = unsafe {
            Vec::from_raw_parts(
                ptr.crystallographic_orbits,
                n_atoms as usize,
                n_atoms as usize,
            )
        };
        let primitive_lattice = ptr.primitive_lattice;
        let mapping_to_primitive = unsafe {
            Vec::from_raw_parts(ptr.mapping_to_primitive, n_atoms as usize, n_atoms as usize)
        };
        let n_std_atoms = ptr.n_std_atoms as i32;
        let std_lattice = ptr.std_lattice;
        let std_types = unsafe {
            Vec::from_raw_parts(ptr.std_types, n_std_atoms as usize, n_std_atoms as usize)
        };
        let std_positions = unsafe {
            Vec::from_raw_parts(
                ptr.std_positions,
                n_std_atoms as usize,
                n_std_atoms as usize,
            )
        };
        let std_rotation_matrix = ptr.std_rotation_matrix;
        let std_mapping_to_primitive = unsafe {
            Vec::from_raw_parts(
                ptr.std_mapping_to_primitive,
                n_std_atoms as usize,
                n_std_atoms as usize,
            )
        };
        let pointgroup_symbol =
            match CString::from(unsafe { CStr::from_ptr(ptr.pointgroup_symbol.as_ptr()) }).to_str()
            {
                Ok(s) => String::from(s),
                Err(_) => return Err(SpglibError::Unknown),
            };
        Ok(Dataset {
            spacegroup_number,
            hall_number,
            international_symbol,
            hall_symbol,
            choice,
            transformation_matrix,
            origin_shift,
            n_operations,
            rotations,
            translations,
            n_atoms,
            wyckoffs,
            site_symmetry_symbols,
            equivalent_atoms,
            crystallographic_orbits,
            primitive_lattice,
            mapping_to_primitive,
            n_std_atoms,
            std_lattice,
            std_types,
            std_positions,
            std_rotation_matrix,
            std_mapping_to_primitive,
            pointgroup_symbol,
        })
    }
}