use openbabel_sys::ob;
use super::molecule;

pub enum FingerprintOpenBabelKind {
    FP2 { nbits: u32 },
    FP3 { nbits: u32 },
    FP4 { nbits: u32 },
    ECFP0 { nbits: u32 },
    ECFP2 { nbits: u32 },
    ECFP4 { nbits: u32 },
    ECFP6 { nbits: u32 },
    ECFP8 { nbits: u32 },
    ECFP10 { nbits: u32 }
}

impl FingerprintOpenBabelKind {
    fn as_str(&self) -> &'static str {
        match self {
            FingerprintOpenBabelKind::FP2 { nbits: _ } => "FP2",
            FingerprintOpenBabelKind::FP3 { nbits: _ } => "FP3",
            FingerprintOpenBabelKind::FP4 { nbits: _ } => "FP4",
            FingerprintOpenBabelKind::ECFP0 { nbits: _ } => "ECFP0",
            FingerprintOpenBabelKind::ECFP2 { nbits: _ } => "ECFP2",
            FingerprintOpenBabelKind::ECFP4 { nbits: _ } => "ECFP4",
            FingerprintOpenBabelKind::ECFP6 { nbits: _ } => "ECFP6",
            FingerprintOpenBabelKind::ECFP8 { nbits: _ } => "ECFP8",
            FingerprintOpenBabelKind::ECFP10 { nbits: _ } => "ECFP10",
        }
    }

    fn get_nbits(&self) -> &u32 {
        match self {
            FingerprintOpenBabelKind::FP2 { nbits } => nbits,
            FingerprintOpenBabelKind::FP3 { nbits } => nbits,
            FingerprintOpenBabelKind::FP4 { nbits } => nbits,
            FingerprintOpenBabelKind::ECFP0 { nbits } => nbits,
            FingerprintOpenBabelKind::ECFP2 { nbits } => nbits,
            FingerprintOpenBabelKind::ECFP4 { nbits } => nbits,
            FingerprintOpenBabelKind::ECFP6 { nbits } => nbits,
            FingerprintOpenBabelKind::ECFP8 { nbits } => nbits,
            FingerprintOpenBabelKind::ECFP10 { nbits } => nbits,
        }
    }
}

pub struct Fingerprint {
    // name: String
    kind: FingerprintOpenBabelKind
}

impl Fingerprint {
    /// Fingerprint FP3 & FP4 require data files of patterns.txt and SMARTS_InteLigand.txt 
    /// If "Open Babel Error in Read PatternFile" is encountered,
    /// setting BABEL_DATADIR to where those files are located will solve the issue.
    // pub fn new(name_fp: &str) -> Self {
    //     Self { name: name_fp.to_string() }
    // }
    pub fn new(kind: FingerprintOpenBabelKind) -> Self {
        Self { kind }
    }

    // pub fn get_fingerprint(&self, mol: &molecule::Molecule, nbits: u32) -> cxx::UniquePtr<cxx::CxxVector<u32>> {
    //     cxx::let_cxx_string!(name = &self.name);
    //     ob::OBFingerprint_get_fingerprint(&name, &mol.ob_mol, nbits) // If nbits <=0, nbits = 4096
    // }
    pub fn get_fingerprint(&self, mol: &molecule::Molecule) -> cxx::UniquePtr<cxx::CxxVector<u32>> {
        cxx::let_cxx_string!(name = &self.kind.as_str());
        ob::OBFingerprint_get_fingerprint(&name, &mol.ob_mol, *self.kind.get_nbits()) // If nbits <=0, nbits = 4096
    }

    pub fn get_fingerprint_in_batch(&self, smiles_vec: &Vec<String>) -> cxx::UniquePtr<cxx::CxxVector<u32>> {
        cxx::let_cxx_string!(name = &self.kind.as_str());
        ob::OBFingerprint_get_fingerprint_in_batch(&name, smiles_vec, *self.kind.get_nbits())
    }
}

#[cfg(test)]
mod test_mod_fingerprint {
    use super::*;

    // #[test]
    // fn test_fingerprint_ecfp() {
    //     for s in vec!["ECFP0", "ECFP2", "ECFP4", "ECFP6", "ECFP8", "ECFP10"] {
    //         let fp = Fingerprint::new(s); 
    //         let mol = molecule::Molecule::new_from_smiles("CCNCC");
    //         let fp_data = fp.get_fingerprint(&mol, 4096);
    //         assert_eq!(fp_data.len(), 128);
    //     }
    // }

    #[test]
    fn test_fingerprint_fp() {
        let mol = molecule::Molecule::new_from_smiles("CCNCC");
        for fp in vec![
            Fingerprint::new(FingerprintOpenBabelKind::FP2 { nbits: 4096 }),
            Fingerprint::new(FingerprintOpenBabelKind::FP3 { nbits: 4096 }),
            Fingerprint::new(FingerprintOpenBabelKind::FP4 { nbits: 4096 })
        ].iter() {
            let fp_data = fp.get_fingerprint(&mol);
            assert_eq!(fp_data.len(), 128);
        }
    }

    #[test]
    fn test_fingerprint_fp_batch() {
        for fp in vec![
            Fingerprint::new(FingerprintOpenBabelKind::FP2 { nbits: 4096 }),
            Fingerprint::new(FingerprintOpenBabelKind::FP3 { nbits: 4096 }),
            Fingerprint::new(FingerprintOpenBabelKind::FP4 { nbits: 4096 })
        ].iter() {
            let smiles_vec = vec![
                String::from("CCNCC"),
                String::from("c1ccccc1")
            ];
            let _fp_data = fp.get_fingerprint_in_batch(&smiles_vec);
            // assert_eq!(fp_data.len(), 128 * 2);
        }
    }

    // #[test]
    // fn test_fingerprint_multiple() {
    //     let fp = Fingerprint::new("FP2");
    //     let mols: Vec<molecule::Molecule> = vec!["CCCC", "CCCN"]
    //         .iter()
    //         .map(|smiles| molecule::Molecule::new_from_smiles(smiles))
    //         .collect();
    //     let fpds: Vec<cxx::UniquePtr<cxx::CxxVector<u32>>> = mols
    //         .iter()
    //         .map(|mol| fp.get_fingerprint(mol, 4096))
    //         .collect(); 
    //     assert_eq!(fpds[0].len(), 128);
    //     assert_eq!(fpds[1].len(), 128);
    // }
}

