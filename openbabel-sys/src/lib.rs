#[cxx::bridge(namespace = "OpenBabel")]
pub mod ob {
    unsafe extern "C++" {
        include!("openbabel-sys/src/wrapper.h");
        type OBMol;
        type OBSmartsPattern;
        type OBFingerprint;

        // OBConversion
        fn OBConversion_smi_to_mol(smiles: &CxxString) -> UniquePtr<OBMol>;

        // OBMol
        fn OBMol_num_atoms(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_num_bonds(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_num_hvy_atoms(mol: &UniquePtr<OBMol>) -> u32;
        fn OBMol_get_mol_wt(mol: &UniquePtr<OBMol>) -> f64;

        // OBFingerprint
        // fn OBFingerprint_find(fp_name: &CxxString) -> UniquePtr<OBFingerprint>;
        fn OBFingerprint_get_fingerprint(fp_name: &CxxString, mol: &UniquePtr<OBMol>, nbits: u32) -> UniquePtr<CxxVector<u32>>;

        // OBSmartsPattern
        fn OBSmartsPattern_new(smarts: &CxxString) -> UniquePtr<OBSmartsPattern>;
        fn OBSmartsPattern_num_atoms(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_num_bonds(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_num_matches(pattern: &UniquePtr<OBSmartsPattern>) -> u32;
        fn OBSmartsPattern_match(pattern: &UniquePtr<OBSmartsPattern>, mol: &UniquePtr<OBMol>) -> UniquePtr<CxxVector<i32>>;
    }
}