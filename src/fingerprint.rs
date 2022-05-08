use openbabel_sys::ob;
use super::molecule;

pub struct Fingerprint {
    name: String
}

impl Fingerprint {
    /// Fingerprint FP3 & FP4 require data files of patterns.txt and SMARTS_InteLigand.txt 
    /// If "Open Babel Error in Read PatternFile" is encountered,
    /// setting BABEL_DATADIR to where those files are located will solve the issue.
    pub fn new(name_fp: &str) -> Self {
        Self { name: name_fp.to_string() }
    }

    pub fn get_fingerprint(&self, mol: &molecule::Molecule, nbits: u32) -> cxx::UniquePtr<cxx::CxxVector<u32>> {
        cxx::let_cxx_string!(name = &self.name);
        ob::OBFingerprint_get_fingerprint(&name, &mol.ob_mol, nbits) // If nbits <=0, nbits = 4096
    }
}

#[cfg(test)]
mod test_mod_fingerprint {
    use super::*;

    #[test]
    fn test_fingerprint_ecfp() {
        for s in vec!["ECFP0", "ECFP2", "ECFP4", "ECFP6", "ECFP8", "ECFP10"] {
            let fp = Fingerprint::new(s); 
            let mol = molecule::Molecule::new_from_smiles("CCNCC");
            let fp_data = fp.get_fingerprint(&mol, 4096);
            assert_eq!(fp_data.len(), 128);
        }
    }

    #[test]
    fn test_fingerprint_fp() {
        for s in vec!["FP2", "FP3", "FP4"] {
            let fp = Fingerprint::new(s); 
            let mol = molecule::Molecule::new_from_smiles("CCNCC");
            let fp_data = fp.get_fingerprint(&mol, 4096);
            assert_eq!(fp_data.len(), 128);
        }
    }
}

