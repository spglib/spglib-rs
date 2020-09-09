//! Description of a crystallographic spacegroup.

use std::convert::TryFrom;
use std::ffi::{CStr, CString};

use spglib_sys as ffi;

use crate::error::SpglibError;

/// Container for a spacegroup's properties
#[derive(Clone, Debug)]
pub struct Spacegroup {
    /// Space group number as defined in the International Tables for Crystallography.
    pub number: i32,
    /// Abbreviated Hermann-Mauguin notation spacegroup symbol.
    pub international_short: String,
    /// Full Hermann-Mauguin notation spacegroup symbol.
    pub international_full: String,
    /// Full Hermann-Mauguin notation spacegroup symbol.
    pub international: String,
    /// Schoenflies notation spacegroup symbol.
    pub schoenflies: String,
    /// Hall symbol.
    pub hall_symbol: String,
    /// Information on unique axis, settings, or cell choices.
    pub choice: String,
    /// Full Hermann-Mauguin notation pointgroup symbol.
    pub pointgroup_international: String,
    /// Schoenflies notation pointgroup symbol.
    pub pointgroup_schoenflies: String,
    /// Arithmetic notation crystal class number.
    pub arithmetic_crystal_class_number: i32,
    /// Arithmetic notation crystal class symbol.
    pub arithmetic_crystal_class_symbol: String,
}

impl TryFrom<ffi::SpglibSpacegroupType> for Spacegroup {
    type Error = SpglibError;

    fn try_from(value: ffi::SpglibSpacegroupType) -> Result<Self, Self::Error> {
        let ptr = value;
        // process fields
        let number = ptr.number as i32;
        let international_short =
            match CString::from(unsafe { CStr::from_ptr(ptr.international_short.as_ptr()) })
                .to_str()
            {
                Ok(s) => String::from(s),
                Err(_) => return Err(SpglibError::Unknown),
            };
        let international_full = match CString::from(unsafe {
            CStr::from_ptr(ptr.international_full.as_ptr())
        })
        .to_str()
        {
            Ok(s) => String::from(s),
            Err(_) => return Err(SpglibError::Unknown),
        };
        let international =
            match CString::from(unsafe { CStr::from_ptr(ptr.international.as_ptr()) }).to_str() {
                Ok(s) => String::from(s),
                Err(_) => return Err(SpglibError::Unknown),
            };
        let schoenflies =
            match CString::from(unsafe { CStr::from_ptr(ptr.schoenflies.as_ptr()) }).to_str() {
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
        let pointgroup_international =
            match CString::from(unsafe { CStr::from_ptr(ptr.pointgroup_international.as_ptr()) })
                .to_str()
            {
                Ok(s) => String::from(s),
                Err(_) => return Err(SpglibError::Unknown),
            };
        let pointgroup_schoenflies =
            match CString::from(unsafe { CStr::from_ptr(ptr.pointgroup_schoenflies.as_ptr()) })
                .to_str()
            {
                Ok(s) => String::from(s),
                Err(_) => return Err(SpglibError::Unknown),
            };
        let arithmetic_crystal_class_number = ptr.arithmetic_crystal_class_number as i32;
        let arithmetic_crystal_class_symbol = match CString::from(unsafe {
            CStr::from_ptr(ptr.arithmetic_crystal_class_symbol.as_ptr())
        })
        .to_str()
        {
            Ok(s) => String::from(s),
            Err(_) => return Err(SpglibError::Unknown),
        };
        Ok(Spacegroup {
            number,
            international_short,
            international_full,
            international,
            schoenflies,
            hall_symbol,
            choice,
            pointgroup_international,
            pointgroup_schoenflies,
            arithmetic_crystal_class_number,
            arithmetic_crystal_class_symbol,
        })
    }
}

impl Spacegroup {
    /// Returns a spacegroup initialized by hall number
    ///
    /// # Example
    ///
    /// Get the spacegroup for a BCC cell (hall number 529).
    ///
    /// ```
    /// use spglib::spacegroup::Spacegroup;
    ///
    /// let group = Spacegroup::from_hall_number(529);
    /// assert_eq!("Im-3m", group.international_short);
    /// ```
    pub fn from_hall_number(hall_number: i32) -> Spacegroup {
        let raw = unsafe { ffi::spg_get_spacegroup_type(hall_number) };
        Spacegroup::try_from(raw).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::spacegroup::Spacegroup;
    #[test]
    fn spacegroup_from_hall_number() {
        let res = Spacegroup::from_hall_number(446);
        assert_eq!(res.number, 156);
        assert_eq!(&res.international_short, "P3m1");
        assert_eq!(&res.international_full, "P 3 m 1");
        assert_eq!(&res.international, "P 3 m 1");
        assert_eq!(&res.schoenflies, "C3v^1");
        assert_eq!(&res.hall_symbol, "P 3 -2\"");
        assert_eq!(&res.choice, "");
        assert_eq!(&res.pointgroup_international, "3m");
        assert_eq!(&res.pointgroup_schoenflies, "C3v");
        assert_eq!(res.arithmetic_crystal_class_number, 45);
        assert_eq!(&res.arithmetic_crystal_class_symbol, "3m1P");
    }
}
