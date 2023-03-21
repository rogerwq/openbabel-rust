//! Openbabel SmartsPattern
//! 
//! 
//! # SMARTS Substructure Search 
//! ```
//! use openbabel::molecule;
//! use openbabel::smartspattern;
//! 
//! let mol = molecule::Molecule::new_from_smiles("NCC(=O)NCC");
//! let sp = smartspattern::SmartsPattern::new_from_smarts("O=CN*");
//! assert_eq!(sp.num_atoms(), 4);
//! assert_eq!(sp.num_bonds(), 3);
//! let match_result = sp.find_match(&mol);
//! assert_eq!(sp.num_matches(), 1);
//! assert_eq!(vec![vec![4, 3, 5, 6]], match_result.as_slice());
//! ```

use ob_rs::ob;
use super::molecule;

pub struct SmartsPattern {
    ob_sp: cxx::UniquePtr<ob::OBSmartsPattern>
}

impl SmartsPattern {
    pub fn new_from_smarts(smarts: &str) -> Self {
        cxx::let_cxx_string!(smarts_cxx = smarts);
        Self { ob_sp: ob::OBSmartsPattern_from_smarts(&smarts_cxx) }
    }

    pub fn num_atoms(&self) -> u32 { ob::OBSmartsPattern_num_atoms(&self.ob_sp) }
    pub fn num_bonds(&self) -> u32 { ob::OBSmartsPattern_num_bonds(&self.ob_sp) }
    pub fn num_matches(&self) -> u32 { ob::OBSmartsPattern_num_matches(&self.ob_sp) }

    pub fn find_match(&self, mol: &molecule::Molecule) -> Vec<Vec<i32>> { // 'match' is keyword in rust, use 'find_match' instead
        ob::OBSmartsPattern_match(&self.ob_sp, &mol.ob_mol)
            .as_slice()
            .split(|&i| i == -1)
            .filter(|v| v.len() > 0)
            .map(|v| v.to_vec())
            .collect::<Vec<Vec<i32>>>()
    }
}

#[cfg(test)]
mod test_mod_smartspattern {
    use super::*;

    #[test]
    fn test_match() {
        let sp = SmartsPattern::new_from_smarts("O=CN*");
        assert_eq!(sp.num_atoms(), 4);
        assert_eq!(sp.num_bonds(), 3);
        let mol_1 = molecule::Molecule::new_from_smiles("NCC(=O)NCC");
        let match_result_1 = sp.find_match(&mol_1);
        assert_eq!(sp.num_matches(), 1);
        assert_eq!(vec![vec![4, 3, 5, 6]], match_result_1);
        let mol_2 = molecule::Molecule::new_from_smiles("NCCNCC");
        let match_result_2 = sp.find_match(&mol_2);
        assert_eq!(sp.num_matches(), 0);
        assert_eq!(0, match_result_2.as_slice().len());
        let mol_3 = molecule::Molecule::new_from_smiles("CNC(=O)C(=O)NCC");
        let match_result_3 = sp.find_match(&mol_3);
        assert_eq!(sp.num_matches(), 2);
        assert_eq!(vec![vec![4, 3, 2, 1], vec![6, 5, 7, 8]], match_result_3.as_slice());
        let test_cases = vec![
            // symmetric query smarts 
            ("COc1cc([N+](=O)[O-])c(OC)cc1CC(C)N", 2, vec![vec![4, 3, 13, 12, 9, 5, 6, 7], vec![9, 12, 13, 3, 4, 5, 6, 7]]), 
            // CHEMBL99990
            ("NCCCNCCCCNC(=O)CCCC(=O)NCCCCCCN=C(N)N", 0, vec![]),
            // CHEMBL99965 
            ("Cc1occc1C(=S)Nc1ccc(Cl)c(/C=N/OC(C)(C)C)c1", 0, vec![]),
            // CHEMBL10030
            ("O=C(c1ccc(OCCN2CCCC2)cc1)c1c(-c2ccc(O)cc2)sc2cc(O)ccc12", 0, vec![])
        ];
        for (s, m_c, m_v) in test_cases.iter() {
            let sp = SmartsPattern::new_from_smarts("c1ccccc1N=O");
            let mol = molecule::Molecule::new_from_smiles(s);
            let match_result = sp.find_match(&mol);
            assert_eq!(sp.num_matches(), *m_c as u32);
            assert_eq!(m_v, match_result.as_slice());
        }
    }
}