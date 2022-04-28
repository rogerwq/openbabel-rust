use openbabel_sys::ob;
use super::molecule;

pub struct SmartsPattern {
    ob_sp: cxx::UniquePtr<ob::OBSmartsPattern>
}

impl SmartsPattern {
    pub fn new(smarts: &str) -> Self {
        cxx::let_cxx_string!(smarts_cxx = smarts);
        Self { ob_sp: ob::OBSmartsPattern_new(&smarts_cxx) }
    }

    pub fn num_atoms(&self) -> u32 { ob::OBSmartsPattern_num_atoms(&self.ob_sp) }
    pub fn num_bonds(&self) -> u32 { ob::OBSmartsPattern_num_bonds(&self.ob_sp) }
    pub fn num_matches(&self) -> u32 { ob::OBSmartsPattern_num_matches(&self.ob_sp) }

    pub fn match_(&self, mol: &molecule::Molecule) -> cxx::UniquePtr<cxx::CxxVector<i32>> { // keyword 'match' in rust
        ob::OBSmartsPattern_match(&self.ob_sp, &mol.ob_mol)
    }
}

#[cfg(test)]
mod test_mod_smartspattern {
    use super::*;
    use super::super::utils;

    #[test]
    fn test_match() {
        let mol = molecule::Molecule::new_from_smiles("NCC(=O)NCC");
        let sp_1 = SmartsPattern::new("O=CN");
        assert_eq!(sp_1.num_atoms(), 3);
        assert_eq!(sp_1.num_bonds(), 2);
        let maplist_1 = sp_1.match_(&mol);
        assert_eq!(sp_1.num_matches(), 1);
        assert_eq!(vec![4, 3, 5], utils::cxx_vector_into_vector(&maplist_1));
        let sp_2 = SmartsPattern::new("CN");
        assert_eq!(sp_2.num_atoms(), 2);
        assert_eq!(sp_2.num_bonds(), 1);
        let maplist_2 = sp_2.match_(&mol);
        assert_eq!(sp_2.num_matches(), 3);
        assert_eq!(vec![2, 1, 3, 5, 6, 5], utils::cxx_vector_into_vector(&maplist_2))
    }
}