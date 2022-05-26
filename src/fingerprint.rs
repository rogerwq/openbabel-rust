//! Openbabel Fingerprint
//! 
//! # Environment variable BABEL_DATA setting
//! 
//! PatternFP fingerprint might require data files as input, e.g. the derivatives FP3 and FP4 require data files of patterns.txt and SMARTS_InteLigand.txt respectively.
//! Set BABEL_DATADIR to where those files are located, if "Open Babel Error in Read PatternFile" is encountered,

use openbabel_sys::ob;
use super::molecule;

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
    fn as_str(&self, thread_id: u32) -> String {
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
        format!("{}_thread_{}", fp_name, thread_id)
    }

    fn get_nbits(&self) -> &u32 {
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

pub struct Fingerprint {
    kind: Kind
}

impl Fingerprint {
    pub fn new(kind: Kind) -> Self {
        Self { kind }
    }

    pub fn get_fingerprint(&self, mol: &molecule::Molecule, thread_id: u32) -> cxx::UniquePtr<cxx::CxxVector<u32>> {
        cxx::let_cxx_string!(name = &self.kind.as_str(thread_id));
        ob::OBFingerprint_get_fingerprint(&name, &mol.ob_mol, *self.kind.get_nbits()) // If nbits <=0, nbits = 4096
    }

    pub fn get_fingerprint_in_batch(&self, smiles_vec: &Vec<String>, thread_id: u32) -> cxx::UniquePtr<cxx::CxxVector<u32>> {
        cxx::let_cxx_string!(name = &self.kind.as_str(thread_id));
        ob::OBFingerprint_get_fingerprint_in_batch(&name, smiles_vec, *self.kind.get_nbits())
    }
}

#[cfg(test)]
mod test_mod_fingerprint {
    use super::*;

    #[test]
    fn test_fingerprint_fp() {
        let mol = molecule::Molecule::new_from_smiles("CCNCC");
        for fp in vec![
            Fingerprint::new(Kind::FP2 { nbits: 4096 }),
            Fingerprint::new(Kind::FP3 { nbits: 4096 }),
            Fingerprint::new(Kind::FP4 { nbits: 4096 })
        ].iter() {
            let fp_data = fp.get_fingerprint(&mol, 0);
            assert_eq!(fp_data.len(), 128);
        }
    }

    #[test]
    fn test_fingerprint_fp_in_batch() {
        for fp in vec![
            Fingerprint::new(Kind::FP2 { nbits: 4096 }),
            Fingerprint::new(Kind::FP3 { nbits: 4096 }),
            Fingerprint::new(Kind::FP4 { nbits: 4096 })
        ].iter() {
            let smiles_vec = vec![
                String::from("CCNCC"),
                String::from("c1ccccc1")
            ];
            let fp_data = fp.get_fingerprint_in_batch(&smiles_vec, 1);
            assert_eq!(fp_data.len(), 128 * 2);
        }
    }

    #[test]
    fn test_fingerprint_ecfp() {
        let mol = molecule::Molecule::new_from_smiles("CCNCC");
        for fp in vec![
            Fingerprint::new(Kind::ECFP0 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP2 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP4 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP6 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP8 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP10 { nbits: 4096 }),
        ].iter() {
            let fp_data = fp.get_fingerprint(&mol, 2);
            assert_eq!(fp_data.len(), 128);
        }
    }

    #[test]
    fn test_fingerprint_ecfp_in_batch() {
        for fp in vec![
            Fingerprint::new(Kind::ECFP0 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP2 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP4 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP6 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP8 { nbits: 4096 }),
            Fingerprint::new(Kind::ECFP10 { nbits: 4096 }),
        ].iter() {
            let smiles_vec = vec![
                String::from("CCNCC"),
                String::from("c1ccccc1")
            ];
            let fp_data = fp.get_fingerprint_in_batch(&smiles_vec, 1);
            assert_eq!(fp_data.len(), 128 * 2);
        }
    }
}

