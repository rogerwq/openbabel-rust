//! Openbabel Molecule
//! 
//! 
//! # Create a Molecule from a SMILES string 
//! ```
//! use openbabel::molecule;
//! 
//! let mol = molecule::Molecule::new_from_smiles("c1ccccc1");
//! assert_eq!(mol.num_atoms(), 6);
//! ```

use crate::conversion::Formats;
use ob_rs::ob;
use std::path::Path;

pub struct Molecule {
    pub ob_mol: cxx::UniquePtr<ob::OBMol>
}

impl Molecule {
    pub fn new_from_smiles(smiles: &str) -> Self {
        cxx::let_cxx_string!(smiles_cxx = smiles);
        Self {
            ob_mol: ob::OBMol_from_smiles(&smiles_cxx),
        }
    }

    /// Creates a new `Molecule` from the inputs of a string.
    /// [InputFormat](crate::conversion::InputFormat) lists the supported formats.
    ///
    /// Only needs `input_format` specified in the [Formats](crate::conversion::Formats) object.
    pub fn new_from_string(input: &str, formats: Formats) -> Result<Self, ()> {
        let ob_mol = ob::OBMol_new();
        cxx::let_cxx_string!(input_cxx = input);
        let read_result = ob::OBConversion_read_string(&formats.ob_conv, &ob_mol, &input_cxx);
        if read_result == 0 {
            return Ok(Self { ob_mol: ob_mol });
        }
        Err(())
    }

    /// Creates a new `Molecule` by reading a file. [InputFormat](crate::conversion::InputFormat)
    /// lists the supported input formats.
    ///
    /// Needs `input_format` specified in the [Formats](crate::conversion::Formats) object.
    pub fn new_from_file<P: AsRef<Path>>(path: P, formats: Formats) -> Result<Self, ()> {
        let ob_mol = ob::OBMol_new();
        cxx::let_cxx_string!(file_path_cxx = path.as_ref().to_str().expect("invalid path"));

        let result = ob::OBConversion_read_file(&formats.ob_conv, &ob_mol, &file_path_cxx);
        if result == 0 {
            return Ok(Self { ob_mol });
        }
        Err(())
    }

    /// Returns `ob_mol` as a [String](std::String) in the specified
    /// [OutputFormat](crate::conversion::OutputFormat) of [Formats](crate::conversion::Formats).
    pub fn write_string(&self, formats: Formats) -> Result<String, ()> {
        if formats.output_format.is_some() {
            return Ok(ob::OBConversion_write_string(
                &formats.ob_conv,
                &self.ob_mol,
            ));
        }
        Err(())
    }

    /// Writes `Molecule` to a file.
    ///
    /// Requires that the [OutputFormat](crate::conversion::OutputFormat) in the
    /// [Formats](crate::conversion::Formats) is specified.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P, formats: Formats) -> Result<(), ()> {
        if formats.input_format.is_some() && formats.output_format.is_some() {
            cxx::let_cxx_string!(file_path_cxx = path.as_ref().to_str().expect("invalid path"));
            let write_result =
                ob::OBConversion_write_file(&formats.ob_conv, &self.ob_mol, &file_path_cxx);
            if write_result == 0 {
                return Ok(());
            }
        }
        Err(())
    }

    pub fn is_valid(&self) -> bool { !self.ob_mol.is_null() }
    pub fn num_atoms(&self) -> u32 { ob::OBMol_num_atoms(&self.ob_mol) }
    pub fn num_bonds(&self) -> u32 { ob::OBMol_num_bonds(&self.ob_mol) }
    pub fn num_hvy_atoms(&self) -> u32 { ob::OBMol_num_hvy_atoms(&self.ob_mol) }
    pub fn get_mol_wt(&self) -> f64 { ob::OBMol_get_mol_wt(&self.ob_mol) }
}

#[cfg(test)]
mod test_mod_molecule {
    use super::*;
    use crate::conversion::{Formats, InputFormat, OutputFormat};

    #[test]
    fn test_molecule() {
        let mol = Molecule::new_from_smiles("CCNCC");
        assert!(mol.is_valid());
        assert_eq!(mol.num_atoms(), 5);
        assert_eq!(mol.num_bonds(), 4);
        assert_eq!(mol.num_hvy_atoms(), 5);
        assert!((mol.get_mol_wt() - 73.137).abs() < 1e-3, "mol wt is {}", mol.get_mol_wt());
    }

    #[test]
    fn test_read_from_string() {
        let formats = Formats::new(Some(InputFormat::xyz), Some(OutputFormat::tmol)).expect("unable to set in- and output formats");
        let input_str = "4

Au  0.00000000  0.00000000  0.00000000
Au  1.44249783  2.49847954  0.00000000
Au  1.44249783  0.83282651  2.35558910
Au  0.00000000  3.33130605  2.35558910";

        let mol = Molecule::new_from_string(input_str, formats).expect("unable to create Molecule from string");
        assert!(mol.is_valid());
        assert_eq!(mol.num_atoms(), 4);
        assert_eq!(mol.num_bonds(), 5);
    }
}
