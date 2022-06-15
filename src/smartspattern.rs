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
//! assert_eq!(vec![4, 3, 5, 6], match_result.as_slice());
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

    pub fn find_match(&self, mol: &molecule::Molecule) -> cxx::UniquePtr<cxx::CxxVector<i32>> { // 'match' is keyword in rust, use 'find_match' instead
        ob::OBSmartsPattern_match(&self.ob_sp, &mol.ob_mol)
    }
}

#[cfg(test)]
mod test_mod_smartspattern {
    use super::*;

    #[test]
    fn test_match() {
        let mol = molecule::Molecule::new_from_smiles("NCC(=O)NCC");
        let sp = SmartsPattern::new_from_smarts("O=CN*");
        assert_eq!(sp.num_atoms(), 4);
        assert_eq!(sp.num_bonds(), 3);
        let match_result = sp.find_match(&mol);
        assert_eq!(sp.num_matches(), 1);
        assert_eq!(vec![4, 3, 5, 6], match_result.as_slice());
    }
}