//!
//!  OpenBabel Rust Bindings
//! 
//! OBMol
//! -----
//! OBMol_num_atoms <-> OBMol::NumAtoms
//! OBMol_num_bonds <-> OBMol::NumBonds
//! OBMol_num_hvy_atoms <-> OBMol::NumHvyAtoms
//! OBMol_get_mol_wt <-> OBMol::GetMolWt
//!
//! OBForceField
//! ------------
//! OBForceField_find_forcefield <-> OBForceField::FindForceField
//! OBForceField_setup <-> OBForceField::Setup
//! OBForceField_conjugate_gradients <-> OBForceField::ConjugateGradients
//! OBForceField_conjugate_gradients_initialize <-> OBForceField::ConjugateGradientsInitialize
//! OBForceField_conjugate_gradients_take_n_steps <-> OBForceField::ConjugateGradientsTakeNSteps
//! OBForceField_steepest_descent <-> OBForceField::SteepestDescent
//! OBForceField_steepest_descent_initialize <-> OBForceField::SteepestDescentInitialize
//! OBForceField_steepest_descent_take_n_steps <-> OBForceField::SteepestDescentTakeNSteps
//! OBForceField_energy <-> OBForceField::Energy
//! OBForceField_is_setup_needed <-> OBForceField::IsSetupNeeded
//!
//!  
//! OBFingerprint
//! -------------
//! OBFingerprint_get_fingerprint <-> OBFingerprint::GetFingerprint
//! 
//! 
//! 
//! OBSmartsPattern
//! ---------------
//! 
//! OBSmartsPattern_from_smarts <-> OBSmartsPattern::Init
//! OBSmartsPattern_num_atoms <-> OBSmartsPattern::NumAtoms
//! OBSmartsPattern_num_bonds <-> OBSmartsPattern::NumBonds
//! OBSmartsPattern_num_matches <-> OBSmartsPattern::NumMatches
//! OBSmartsPattern_match <-> OBSmartsPattern::Match

#[cxx::bridge(namespace = "OpenBabel")]
pub mod ob {
    unsafe extern "C++" {
        include!("openbabel-sys/src/wrapper.h");
        type OBMol;
        type OBSmartsPattern;
        type OBForceField;

        // Debug
        fn print_global_instances();

        // OBConversion
        // fn OBConversion_smi_to_mol(smiles: &CxxString) -> UniquePtr<OBMol>;

        // OBForceField
        fn OBForceField_find_forcefield(ff_name: &CxxString) -> UniquePtr<OBForceField>;
        fn OBForceField_setup(mol: &UniquePtr<OBMol>, pFF: &UniquePtr<OBForceField>) -> u32;
        fn OBForceField_conjugate_gradients(pFF: &UniquePtr<OBForceField>, steps: u32, econv: f64);
        fn OBForceField_conjugate_gradients_initialize(
            pFF: &UniquePtr<OBForceField>,
            steps: u32,
            econv: f64,
        );
        fn OBForceField_conjugate_gradients_take_n_steps(
            pFF: &UniquePtr<OBForceField>,
            n: u32,
        ) -> bool;
        fn OBForceField_steepest_descent(pFF: &UniquePtr<OBForceField>, steps: u32, econv: f64);
        fn OBForceField_steepest_descent_initialize(
            pFF: &UniquePtr<OBForceField>,
            steps: u32,
            econv: f64,
        );
        fn OBForceField_steepest_descent_take_n_steps(
            pFF: &UniquePtr<OBForceField>,
            n: u32,
        ) -> bool;
        fn OBForceField_energy(pFF: &UniquePtr<OBForceField>) -> f64;
        fn OBForceField_is_setup_needed(
            pFF: &UniquePtr<OBForceField>,
            mol: &UniquePtr<OBMol>,
        ) -> bool;

        // OBMol
        fn OBMol_from_smiles(smiles: &CxxString) -> UniquePtr<OBMol>;
        fn OBMol_num_atoms(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_num_bonds(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_num_hvy_atoms(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_get_mol_wt(mol: &UniquePtr<OBMol>) -> f64;

        // OBFingerprint
        fn OBFingerprint_get_fingerprint(fp_name: &CxxString, mol: &UniquePtr<OBMol>, nbits: u32) -> UniquePtr<CxxVector<u32>>;

        // OBSmartsPattern
        fn OBSmartsPattern_from_smarts(smarts: &CxxString) -> UniquePtr<OBSmartsPattern>;
        fn OBSmartsPattern_num_atoms(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_num_bonds(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_num_matches(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_match(pattern: &UniquePtr<OBSmartsPattern>, mol: &UniquePtr<OBMol>) -> UniquePtr<CxxVector<i32>>;
    }
}


mod tests;