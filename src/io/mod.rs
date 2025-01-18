pub mod formats;
pub mod conversion;

use std::path::Path;
use crate::molecule::{ToMol, Molecule};
use ob_rs::ob;

struct StringForMol {
    string: String,
    format: formats::InputFormat
}

impl StringForMol {
    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }

}

/// Creates a new `Molecule` from the inputs of a string.
/// [InputFormat](crate::io::formats::InputFormat) lists the supported formats.
///
impl ToMol for StringForMol {
    fn to_mol(&self, conv: &conversion::Conversion) -> Result<Molecule, ()> {
        let ob_mol = ob::OBMol_new();
        cxx::let_cxx_string!(input_cxx = self.as_str());
        cxx::let_cxx_string!(input_format_cxx = self.format.to_string());
        ob::OBConversion_set_in_format(&conv.ob_conv, &input_format_cxx);
        let read_succesful = ob::OBConversion_read_string(&conv.ob_conv, &ob_mol, &input_cxx);
        if read_succesful {
            return Ok(Molecule { ob_mol: ob_mol });
        }
        Err(())
    }
}

struct FileForMol<'a> {
    path_ref: &'a Path, 
    format: formats::InputFormat
}

/// Creates a new `Molecule` by reading a file. [InputFormat](crate::io::formats::InputFormat)
/// lists the supported input formats.
///
impl ToMol for FileForMol<'_> {
    fn to_mol(&self, conv: &conversion::Conversion) -> Result<Molecule, ()> {
        let ob_mol = ob::OBMol_new();
        cxx::let_cxx_string!(file_path_cxx = self.path_ref.to_str().expect("invalid path"));
        cxx::let_cxx_string!(input_format_cxx = self.format.to_string());
        ob::OBConversion_set_in_format(&conv.ob_conv, &input_format_cxx);
        let read_succesful = ob::OBConversion_read_file(&conv.ob_conv, &ob_mol, &file_path_cxx);
        if read_succesful {
            return Ok(Molecule { ob_mol });
        }
        Err(())
    }
}


#[cfg(test)]
mod test_io {
    use super::*;

    fn test_string(conv: &conversion::Conversion) {
        let string = "4

Au  0.00000000  0.00000000  0.00000000
Au  1.44249783  2.49847954  0.00000000
Au  1.44249783  0.83282651  2.35558910
Au  0.00000000  3.33130605  2.35558910".to_string();

        let format = formats::InputFormat::xyz;
        let string_for_mol = StringForMol { string, format };
        let mol = string_for_mol.to_mol(conv).expect("unable to create Molecule from string");
        assert!(mol.is_valid());
        assert_eq!(mol.num_atoms(), 4);
        assert_eq!(mol.num_bonds(), 5);

        let format_out = formats::OutputFormat::smi;
        assert_eq!(mol.to_string(&conv, &format_out), "[Au]1[Au]2[Au]1[Au]2\t\n");
    }

    #[test]
    fn test_string_for_mol() {
        let conv = conversion::Conversion::new();
        test_string(&conv);
    }

    fn test_file(conv: &conversion::Conversion) {
        let path_ref = std::path::Path::new("./openbabel-sys/openbabel/test/files/3o8g_uff.sdf").as_ref();
        let format = formats::InputFormat::sdf;
        let file_for_mol = FileForMol { path_ref, format };
        let mol = file_for_mol.to_mol(conv).expect("unable to create Molecule from file");
        assert!(mol.is_valid());
        assert_eq!(mol.num_atoms(), 36);
        assert_eq!(mol.num_bonds(), 38);
    }

    #[test]
    fn test_file_for_mol() {
        let conv = conversion::Conversion::new();
        test_file(&conv);
    }

    #[test]
    fn test_reuse_conv() {
        let conv = conversion::Conversion::new();
        test_string(&conv);
        test_file(&conv);
    }

    fn test_pdb(conv: &conversion::Conversion) {
        let path_ref =
            std::path::Path::new("./openbabel-sys/openbabel/test/files/00T_ideal.pdb").as_ref();
        let format = formats::InputFormat::pdb;
        let file_for_mol = FileForMol { path_ref, format };
        let mol = file_for_mol
            .to_mol(conv)
            .expect("unable to create Molecule from file");
        assert!(mol.is_valid());
        assert_eq!(mol.num_atoms(), 22);
        assert_eq!(mol.num_bonds(), 22);
    }

    #[test]
    fn test_file_for_pdb() {
        let conv = conversion::Conversion::new();
        test_pdb(&conv);
    }
}
