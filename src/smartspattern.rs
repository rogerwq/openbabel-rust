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

    // pub fn find_match(&self, mol: &molecule::Molecule) -> cxx::UniquePtr<cxx::CxxVector<i32>> { // 'match' is keyword in rust, use 'find_match' instead
    pub fn find_match(&self, mol: &molecule::Molecule) -> Vec<Vec<i32>> { // 'match' is keyword in rust, use 'find_match' instead
        ob::OBSmartsPattern_match(&self.ob_sp, &mol.ob_mol)
            .as_slice()
            // .iter()
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
        // query smarts is symmetric
        let sp_4 = SmartsPattern::new_from_smarts("c1ccccc1N=O");
        let mol_4 = molecule::Molecule::new_from_smiles("COc1cc([N+](=O)[O-])c(OC)cc1CC(C)N");
        let match_result_4 = sp_4.find_match(&mol_4);
        assert_eq!(sp_4.num_matches(), 2);
        assert_eq!(vec![vec![4, 3, 13, 12, 9, 5, 6, 7], vec![9, 12, 13, 3, 4, 5, 6, 7]], match_result_4.as_slice());
    }
}