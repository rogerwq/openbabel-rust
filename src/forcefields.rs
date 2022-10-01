//! OpenBabel ForceField
//!
//! # Create a Molecule from a SMILES string and optimize it
//! ```
//! use openbabel::molecule;
//! use openbabel::forcefields::{ForceField, ForceFieldVariant, Method};
//!
//! let mut mol = molecule::Molecule::new_from_smiles("c1ccccc1");
//! let force_field = ForceField::new(ForceFieldVariant::GAFF, &mut mol);
//! force_field.optimize(Method::ConjugateGradient, 4, 1e-5);
//! ```

use crate::molecule::Molecule;
use ob_rs::ob;

#[derive(Eq, PartialEq)]
#[allow(non_snake_case)]
pub enum ForceFieldVariant {
    /// Generalized Amber Force Field
    GAFF,

    /// Ghemical
    Ghemical,

    /// Merck Molecular Force Field
    MMFF94,

    /// Merck Molecular Force Field (static variant)
    MMFF94s,

    /// Universal Force Field
    UFF,
}

impl ForceFieldVariant {
    #[allow(non_snake_case)]
    pub fn as_str(&self) -> String {
        let ff_name = match self {
            ForceFieldVariant::GAFF => "gaff",
            ForceFieldVariant::Ghemical => "ghemical",
            ForceFieldVariant::MMFF94 => "mmff94",
            ForceFieldVariant::MMFF94s => "mmff94s",
            ForceFieldVariant::UFF => "uff",
        };
        format!("{}", ff_name)
    }
}

pub enum Method {
    ConjugateGradient,
    SteepestDescent,
}

pub struct ForceField<'a> {
    pub variant: ForceFieldVariant,
    pub mol: &'a mut Molecule,

    ff: cxx::UniquePtr<ob::OBForceField>,
}

impl<'a> ForceField<'a> {
    /// We take a mutable reference to `mol` to make sure that no one else is optimizing the
    /// structure of the molecule while we're doing it.
    pub fn new(force_field: ForceFieldVariant, mol: &'a mut Molecule) -> Self {
        cxx::let_cxx_string!(ff_name_cxx = force_field.as_str());
        let ff = ob::OBForceField_find_forcefield(&ff_name_cxx);

        Self {
            variant: force_field,
            mol,
            ff,
        }
    }

    /// Optimizes the attached molecule with [Method] for a maximum of `max_steps` or until
    /// energy is below `nconv`.
    pub fn optimize(&self, method: Method, max_steps: u32, nconv: f64) {
        if ob::OBForceField_is_setup_needed(&self.ff, &self.mol.ob_mol) {
            ob::OBForceField_setup(&self.mol.ob_mol, &self.ff);
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

    /// Initializes the force field.
    ///
    /// The molecule will be optimized with [Method] for a maximum of `max_steps` or until
    /// energy is below `nconv`.
    ///
    /// Must be called prior to calling [optimize_n_steps](ForceField::optimize_n_steps).
    pub fn initialize_force_field(&self, method: Method, max_steps: u32, nconv: f64) {
        if ob::OBForceField_is_setup_needed(&self.ff, &self.mol.ob_mol) {
            ob::OBForceField_setup(&self.mol.ob_mol, &self.ff);
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
    pub fn optimize_n_steps(&self, method: Method, n: u32) -> bool {
        match method {
            Method::ConjugateGradient => {
                ob::OBForceField_conjugate_gradients_take_n_steps(&self.ff, n)
            }
            Method::SteepestDescent => ob::OBForceField_steepest_descent_take_n_steps(&self.ff, n),
        }
    }

    /// Gets the energy of the attached molecule in the specified force field.
    pub fn energy(&self) -> f64 {
        ob::OBForceField_energy(&self.ff)
    }
}

#[cfg(test)]
mod test_mod_forcefields {
    use super::*;

    #[test]
    fn test_direct_optimization() {
        for ff in [
            ForceFieldVariant::GAFF,
            ForceFieldVariant::Ghemical,
            ForceFieldVariant::MMFF94,
            ForceFieldVariant::MMFF94s,
            ForceFieldVariant::UFF,
        ] {
            let mut mol = Molecule::new_from_smiles("cc");
            let force_field = ForceField::new(ff, &mut mol);
            force_field.optimize(Method::ConjugateGradient, 4, 1e-5);
            assert!(force_field.energy() < 1e-5);
        }
    }

    #[test]
    fn test_stepwise_optimization() {
        let mut mol = Molecule::new_from_smiles("S1SSSSSSS1");
        let force_field = ForceField::new(ForceFieldVariant::GAFF, &mut mol);
        force_field.initialize_force_field(Method::ConjugateGradient, 8, 1e-5);
        let mut count = 0;
        while force_field.optimize_n_steps(Method::ConjugateGradient, 1) {
            count += 1;
        }
        assert!(count > 0);
    }
}
