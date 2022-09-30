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

use crate::forcefields::{ForceField, Method};
use ob_rs::ob;

pub struct Molecule {
    pub ob_mol: cxx::UniquePtr<ob::OBMol>,
    ff_name: ForceField,
    ff: cxx::UniquePtr<ob::OBForceField>,
}

impl Molecule {
    pub fn new_from_smiles(smiles: &str) -> Self {
        cxx::let_cxx_string!(smiles_cxx = smiles);
        Self {
            ob_mol: ob::OBMol_from_smiles(&smiles_cxx),
            ff_name: ForceField::None,
            ff: cxx::UniquePtr::null(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.ob_mol.is_null()
    }
    pub fn num_atoms(&self) -> u32 {
        ob::OBMol_num_atoms(&self.ob_mol)
    }
    pub fn num_bonds(&self) -> u32 {
        ob::OBMol_num_bonds(&self.ob_mol)
    }
    pub fn num_hvy_atoms(&self) -> u32 {
        ob::OBMol_num_hvy_atoms(&self.ob_mol)
    }
    pub fn get_mol_wt(&self) -> f64 {
        ob::OBMol_get_mol_wt(&self.ob_mol)
    }

    // Optimization routines
    pub fn optimize(
        &mut self,
        force_field: ForceField,
        method: Method,
        max_steps: u32,
        nconv: f64,
    ) {
        if self.ff_name == ForceField::None {
            cxx::let_cxx_string!(ff_name_cxx = force_field.as_str());
            self.ff = ob::OBForceField_find_forcefield(&ff_name_cxx);
            self.ff_name = force_field;
        }

        if ob::OBForceField_is_setup_needed(&self.ff, &self.ob_mol) {
            ob::OBForceField_setup(&self.ob_mol, &self.ff);
        }
        match method {
            Method::ConjugateGradient => {
                ob::OBForceField_conjugate_gradients(&self.ff, max_steps, nconv);
            }
            Method::SteepestDescent => {
                ob::OBForceField_steepest_descent(&self.ff, max_steps, nconv);
            }
        }
    }

    /// Initializes the force field. Must be prior to calling [`optimize_n_step`]
    pub fn initialize_force_field(
        &mut self,
        force_field: ForceField,
        method: Method,
        max_steps: u32,
        nconv: f64,
    ) {
        if self.ff_name == ForceField::None {
            cxx::let_cxx_string!(ff_name_cxx = force_field.as_str());
            self.ff = ob::OBForceField_find_forcefield(&ff_name_cxx);
            self.ff_name = force_field;
        }
        if ob::OBForceField_is_setup_needed(&self.ff, &self.ob_mol) {
            ob::OBForceField_setup(&self.ob_mol, &self.ff);
        }
        match method {
            Method::ConjugateGradient => {
                ob::OBForceField_conjugate_gradients_initialize(&self.ff, max_steps, nconv);
            }
            Method::SteepestDescent => {
                ob::OBForceField_steepest_descent_initialize(&self.ff, max_steps, nconv);
            }
        }
    }

    /// Optimizes the molecule `n` steps. Is recommended by OpenBabel if you wish to do stuff in
    /// between optimization steps as you avoid calling initialization every iteration.
    pub fn optimize_n_steps(&mut self, method: Method, n: u32) -> Result<bool, ()> {
        if self.ff_name == ForceField::None {
            return Err(());
        }
        match method {
            Method::ConjugateGradient => Ok(ob::OBForceField_conjugate_gradients_take_n_steps(
                &self.ff, n,
            )),
            Method::SteepestDescent => {
                Ok(ob::OBForceField_steepest_descent_take_n_steps(&self.ff, n))
            }
        }
    }

    pub fn energy(&self) -> Option<f64> {
        match self.ff_name {
            ForceField::None => None,
            _ => Some(ob::OBForceField_energy(&self.ff)),
        }
    }
}

#[cfg(test)]
mod test_mod_molecule {
    use super::*;

    #[test]
    fn test_molecule() {
        let mol = Molecule::new_from_smiles("CCNCC");
        assert!(mol.is_valid());
        assert_eq!(mol.num_atoms(), 5);
        assert_eq!(mol.num_bonds(), 4);
        assert_eq!(mol.num_hvy_atoms(), 5);
        assert!(
            (mol.get_mol_wt() - 73.137).abs() < 1e-3,
            "mol wt is {}",
            mol.get_mol_wt()
        );
    }

    #[test]
    fn test_direct_optimization() {
        for ff in [
            ForceField::GAFF,
            ForceField::Ghemical,
            ForceField::MMFF94,
            ForceField::MMFF94s,
            ForceField::UFF,
        ] {
            let mut mol = Molecule::new_from_smiles("cc");
            mol.optimize(ff, Method::ConjugateGradient, 4, 1e-5);
            assert!(mol.energy().unwrap() < 1e-5);
        }
    }

    #[test]
    fn test_stepwise_optimization() {
        let mut mol = Molecule::new_from_smiles("S1SSSSSSS1");
        mol.initialize_force_field(ForceField::GAFF, Method::ConjugateGradient, 8, 1e-5);
        let mut count = 0;
        while mol.optimize_n_steps(Method::ConjugateGradient, 1).unwrap() {
            count += 1;
        }
        assert!(count > 0);
    }
}
