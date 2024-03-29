//! Openbabel Fingerprint
//! 
//! # set environmental variable BABEL_DATADIR 
//! 
//! PatternFP (fingerprint3.cpp) requires data files as input.
//! E.g. the FP3 and FP4 require data files of patterns.txt and SMARTS_InteLigand.txt respectively.
//! Set BABEL_DATADIR to where those files locate, if "Open Babel Error in Read PatternFile" is encountered.
//! 
//! 
//! 
//! # Use FingerprintGenerator to calculate fingerprints for molecules
//! ```
//! use openbabel::fingerprint;
//! use openbabel::molecule;
//! use openbabel::fingerprint::Kind;
//! 
//! let fpg = fingerprint::FingerprintGenerator::new(Kind::ECFP4 { nbits: 2048 });
//! 
//! let mol = molecule::Molecule::new_from_smiles("c1ccccc1");
//! let fpd = fpg.get_fingerprint(&mol);
//! assert_eq!(fpd.len(), 64);
//! 
//! let aspirin = String::from("O=C(C)Oc1ccccc1C(=O)O");
//! let seroquel = String::from("N1=C(c3c(Sc2c1cccc2)cccc3)N4CCN(CCOCCO)CC4");
//! let lipitor = String::from("O=C(O)C[C@H](O)C[C@H](O)CCn2c(c(c(c2c1ccc(F)cc1)c3ccccc3)C(=O)Nc4ccccc4)C(C)C");
//! let tylenol = String::from("CC(=O)Nc1ccc(O)cc1");
//! let smiles_vec = vec![aspirin, seroquel, lipitor, tylenol];
//! 
//! let fpd_of_mols = fpg.get_fingerprint_for_smiles_vec(&smiles_vec);
//! assert_eq!(fpd_of_mols.len(), 4);
//! for i in 0..4 {
//!     assert_eq!(fpd_of_mols[i].len(), 64);
//! }
//! 
//! ```

use ob_rs::ob;
use super::molecule;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Kind {
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

impl Kind {
    fn as_str(&self) -> String {
        let fp_name = match self {
            Kind::FP2 { nbits: _ } => "FP2",
            Kind::FP3 { nbits: _ } => "FP3",
            Kind::FP4 { nbits: _ } => "FP4",
            Kind::ECFP0 { nbits: _ } => "ECFP0",
            Kind::ECFP2 { nbits: _ } => "ECFP2",
            Kind::ECFP4 { nbits: _ } => "ECFP4",
            Kind::ECFP6 { nbits: _ } => "ECFP6",
            Kind::ECFP8 { nbits: _ } => "ECFP8",
            Kind::ECFP10 { nbits: _ } => "ECFP10",
        };
        format!("{}", fp_name)
    }

    pub fn get_nbits(&self) -> &u32 {
        match self {
            Kind::FP2 { nbits } => nbits,
            Kind::FP3 { nbits } => nbits,
            Kind::FP4 { nbits } => nbits,
            Kind::ECFP0 { nbits } => nbits,
            Kind::ECFP2 { nbits } => nbits,
            Kind::ECFP4 { nbits } => nbits,
            Kind::ECFP6 { nbits } => nbits,
            Kind::ECFP8 { nbits } => nbits,
            Kind::ECFP10 { nbits } => nbits,
        }
    }
}

pub struct FingerprintGenerator {
    kind: Kind
}

impl FingerprintGenerator {
    pub fn new(kind: Kind) -> Self {
        Self { kind }
    }

    // pub fn get_fingerprint(&self, mol: &molecule::Molecule) -> cxx::UniquePtr<cxx::CxxVector<u32>> {
    pub fn get_fingerprint(&self, mol: &molecule::Molecule) -> Vec<u32> {
        cxx::let_cxx_string!(fp_name = &self.kind.as_str());
        ob::OBFingerprint_get_fingerprint(&fp_name, &mol.ob_mol, *self.kind.get_nbits()) // If nbits <=0, nbits = 4096
            .iter().cloned().collect()
    }

    pub fn get_fingerprint_for_smiles_vec(&self, smiles_vec: &Vec<String>) -> Vec<Vec<u32>> {
        smiles_vec.iter()
            .map(|smiles| {
                let mol = molecule::Molecule::new_from_smiles(smiles);
                // self.get_fingerprint(&mol).iter().cloned().collect()
                self.get_fingerprint(&mol)
            })
            .collect()
    }
}


#[cfg(test)]
mod test_mod_fingerprint {
    use super::*;

    // #[test]
    // #[ignore]
    // fn test_error_cases() {
    //     let thread_id = 0;
    //     // FP generated by get_fingerprint() and get_fingerprint_in_batch() are different for the same SMILES "CCCN"
    //     // Root causes to be investigated
    //     let fpk = Kind::FP2 { nbits: 4096 };
    //     let fpds = get_fingerprint_in_batch(&vec![String::from("CCCC"), String::from("CCCN")], &fpk, thread_id);
    //     let fpd = get_fingerprint(&String::from("CCCN"), &fpk, thread_id);
    //     for i in 0..128 {
    //         let offset: usize = 128;
    //         println!("{}: {} - {}", i, fpds.as_ref().unwrap().get(i + offset).unwrap(), fpd.as_ref().unwrap().get(i).unwrap());
    //         assert_eq!(fpds.as_ref().unwrap().get(i + offset).unwrap(), fpd.as_ref().unwrap().get(i).unwrap());
    //     }
    // }

    #[test]
    fn test_get_fp() {
        for fpk in vec![
            Kind::FP2 { nbits: 4096 },
            Kind::FP3 { nbits: 4096 },
            Kind::FP4 { nbits: 4096 },
            Kind::ECFP0 { nbits: 4096 },
            Kind::ECFP2 { nbits: 4096 },
            Kind::ECFP4 { nbits: 4096 },
            Kind::ECFP6 { nbits: 4096 },
            Kind::ECFP8 { nbits: 4096 },
            Kind::ECFP10 { nbits: 4096 },
        ].iter() {
            let mol = molecule::Molecule::new_from_smiles("CCNCC");
            let fpg = FingerprintGenerator::new(fpk.clone());
            let fpd = fpg.get_fingerprint(&mol);
            assert_eq!(fpd.len(), 128);
        }
    }

    #[test]
    fn test_get_fp_for_smiles_vec() {
        let smiles_1 = String::from("CCNCC");
        let smiles_2 = String::from("c1ccccc1");
        let smiles_vec = vec![
            smiles_1, smiles_2
        ];
        for fpk in vec![
            Kind::FP2 { nbits: 4096 },
            Kind::FP3 { nbits: 4096 },
            Kind::FP4 { nbits: 4096 },
            Kind::ECFP0 { nbits: 4096 },
            Kind::ECFP2 { nbits: 4096 },
            Kind::ECFP4 { nbits: 4096 },
            Kind::ECFP6 { nbits: 4096 },
            Kind::ECFP8 { nbits: 4096 },
            Kind::ECFP10 { nbits: 4096 },
        ].iter() {
            let fpg = FingerprintGenerator::new(fpk.clone());
            let fpd_vec = fpg.get_fingerprint_for_smiles_vec(&smiles_vec);
            assert_eq!(fpd_vec.len(), 2);
            assert_eq!(fpd_vec[0].len(), 128);
            assert_eq!(fpd_vec[1].len(), 128);
        }
    }
}

