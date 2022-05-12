#[cxx::bridge(namespace = "OpenBabel")]
pub mod ob {
    unsafe extern "C++" {
        include!("openbabel-sys/src/wrapper.h");
        type OBMol;
        type OBSmartsPattern;
        // type OBFingerprint;

        // OBConversion
        fn OBConversion_smi_to_mol(smiles: &CxxString) -> UniquePtr<OBMol>;

        // OBMol
        fn OBMol_num_atoms(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_num_bonds(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_num_hvy_atoms(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_get_mol_wt(mol: &UniquePtr<OBMol>) -> f64;

        // OBFingerprint
        // type fingerprint2;
        // fn OBFingerprint_find(fp_name: &CxxString) -> UniquePtr<OBFingerprint>;
        // fn OBFingerprint_get_fingerprint2_instance() -> UniquePtr<fingerprint2>;
        // fn OBFingerprint_fingerprint2_get_fingerprint(fp: &UniquePtr<fingerprint2>, mol: &UniquePtr<OBMol>, nbits: u32) -> UniquePtr<CxxVector<u32>>;
        fn OBFingerprint_get_fingerprint(fp_name: &CxxString, mol: &UniquePtr<OBMol>, nbits: u32) -> UniquePtr<CxxVector<u32>>;
        fn OBFingerprint_get_fingerprint_in_batch(fp_name: &CxxString, smiles_vec: &Vec<String>, nbits: u32) -> UniquePtr<CxxVector<u32>>;

        // OBSmartsPattern
        fn OBSmartsPattern_new(smarts: &CxxString) -> UniquePtr<OBSmartsPattern>;
        fn OBSmartsPattern_num_atoms(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_num_bonds(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_num_matches(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_match(pattern: &UniquePtr<OBSmartsPattern>, mol: &UniquePtr<OBMol>) -> UniquePtr<CxxVector<i32>>;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fingerprint_fp() {
        cxx::let_cxx_string!(smiles = "c1ccccc1");
        let mol = ob::OBConversion_smi_to_mol(&smiles);
        for fp_name in vec!["FP2", "FP3", "FP4"] {
            cxx::let_cxx_string!(name = fp_name);
            let p_data = ob::OBFingerprint_get_fingerprint(&name, &mol, 4096);
            assert_eq!(p_data.as_ref().unwrap().len(), 128);
        }
    }
}