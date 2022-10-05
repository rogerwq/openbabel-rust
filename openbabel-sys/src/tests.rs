//!
//! Test cases for OpenBabel Wrapper
//!
//! Only support single-thread 
//! cargo test -- --test-threads=1


#[cfg(test)]
mod test {
    use crate::ob;

    #[macro_export]
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            assert!(($x - $y).abs() < $d, "abs({} - {}) > {}", $x, $y, $d);
        };
    }

    # [test]
    fn test_mol() {
        let test_data: Vec<(String, (u32, u32, u32, f64))> = vec![
            (String::from("c1ccccc1N"), (7, 7, 7, 93.126)),
            (String::from("CCC(COC(=O)[C@@H](NP(=O)(Oc1ccccc1)OC[C@H]1O[C@@]([C@@H]([C@@H]1O)O)(C#N)c1ccc2n1ncnc2N)C)CC"), (42, 45, 42, 602.576))
        ];
        for (s, r) in test_data.iter() {
            cxx::let_cxx_string!(smiles = s.as_str());
            let mol = ob::OBMol_from_smiles(&smiles);
            assert_eq!(ob::OBMol_num_atoms(&mol), r.0);
            assert_eq!(ob::OBMol_num_bonds(&mol), r.1);
            assert_eq!(ob::OBMol_num_hvy_atoms(&mol), r.2);
            assert_delta!(ob::OBMol_get_mol_wt(&mol), r.3, 1e-3);
        }
    }

    #[test]
    fn test_fingerprint() {
        cxx::let_cxx_string!(smiles = "c1ccccc1");
        let mol = ob::OBMol_from_smiles(&smiles);
        for fp_name in vec![
            "FP2", "FP3", "FP4",
            "ECFP0", "ECFP2", "ECFP4", "ECFP6", "ECFP8", "ECFP10",
            ] {
            cxx::let_cxx_string!(name = fp_name);
            let p_data = ob::OBFingerprint_get_fingerprint(&name, &mol, 4096);
            assert_eq!(p_data.as_ref().unwrap().len(), 128);
        }
    }

    #[test]
    fn test_smarts_pattern() {
        cxx::let_cxx_string!(smiles = "NCC(=O)NCC");
        let mol = ob::OBMol_from_smiles(&smiles);
        let test_data: Vec<(String, (u32, u32, u32, Vec<i32>))> = vec![
            (String::from("O=CN"), (3, 2, 1, vec![4, 3, 5])),
            (String::from("CN"), (2, 1, 3, vec![2, 1, 3, 5, 6, 5])),
        ];

        for (s, (num_atoms, num_bonds, num_match, match_indexes)) in test_data.iter() {
            cxx::let_cxx_string!(smarts = s);
            let sp = ob::OBSmartsPattern_from_smarts(&smarts);
            assert_eq!(ob::OBSmartsPattern_num_atoms(&sp), *num_atoms);
            assert_eq!(ob::OBSmartsPattern_num_bonds(&sp), *num_bonds);
            let match_cxx_vec = ob::OBSmartsPattern_match(&sp, &mol);
            assert_eq!(ob::OBSmartsPattern_num_matches(&sp), *num_match);
            assert_eq!(match_cxx_vec.as_slice(), match_indexes);
        }
    }

    #[test]
    fn test_optimize_directly() {
        for ff_name in ["mmff94", "mmff94s", "uff", "gaff", "ghemical"] {
            cxx::let_cxx_string!(ff_name_cxx = ff_name);
            let ff = ob::OBForceField_find_forcefield(&ff_name_cxx);

            cxx::let_cxx_string!(smiles_cxx = "cc");
            let mol = ob::OBMol_from_smiles(&smiles_cxx);

            ob::OBForceField_setup(&mol, &ff);
            let beginning_energy = ob::OBForceField_energy(&ff);

            ob::OBForceField_conjugate_gradients(&ff, 16, 1e-5);
            let end_energy = ob::OBForceField_energy(&ff);
            assert!(beginning_energy > end_energy);
        }
    }

    #[test]
    fn test_steepest_descent() {
        cxx::let_cxx_string!(ff_name = "mmff94");
        let ff = ob::OBForceField_find_forcefield(&ff_name);

        cxx::let_cxx_string!(smiles = "cc");
        let mol = ob::OBMol_from_smiles(&smiles);

        ob::OBForceField_setup(&mol, &ff);
        assert!(ob::OBForceField_energy(&ff) > 4000.0);

        ob::OBForceField_steepest_descent_initialize(&ff, 100, 1e-5);
        assert!(ob::OBForceField_steepest_descent_take_n_steps(&ff, 10) == false);
        assert!(ob::OBForceField_energy(&ff) < 0.01);
    }

    #[test]
    fn test_conjugate_gradient() {
        cxx::let_cxx_string!(ff_name = "mmff94");
        let ff = ob::OBForceField_find_forcefield(&ff_name);

        cxx::let_cxx_string!(smiles = "cc");
        let mol = ob::OBMol_from_smiles(&smiles);

        ob::OBForceField_setup(&mol, &ff);
        assert!(ob::OBForceField_energy(&ff) > 4000.0);

        ob::OBForceField_conjugate_gradients_initialize(&ff, 100, 1e-5);
        assert!(ob::OBForceField_conjugate_gradients_take_n_steps(&ff, 10) == false);
        assert!(ob::OBForceField_energy(&ff) < 0.01);
    }

    #[test]
    fn test_is_setup_needed() {
        cxx::let_cxx_string!(ff_name = "uff");
        let ff1 = ob::OBForceField_find_forcefield(&ff_name);

        cxx::let_cxx_string!(ff_name = "uff");
        let ff2 = ob::OBForceField_find_forcefield(&ff_name);

        cxx::let_cxx_string!(smiles = "[No][Cd][Es]");
        let mol = ob::OBMol_from_smiles(&smiles);

        assert!(ob::OBForceField_is_setup_needed(&ff1, &mol));

        ob::OBForceField_setup(&mol, &ff1);
        assert!(ob::OBForceField_is_setup_needed(&ff1, &mol) == false);

        assert!(ob::OBForceField_is_setup_needed(&ff2, &mol));
    }
}
