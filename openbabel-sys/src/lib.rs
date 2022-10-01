//!  OpenBabel Rust Bindings
//!
//!  OBConversion
//!  ------------
//! OBConversion_new <-> new OBConversion()
//! OBConversion_set_in_format <-> OBConversion::SetInFormat
//! OBConversion_set_out_format <-> OBConversion::SetOutFormat
//! OBConversion_set_in_and_out_formats <-> OBConversion::SetInAndOutFormats
//! OBConversion_read_string <-> OBConversion::ReadString
//! OBConversion_write_string <-> OBConversion::WriteString
//! OBConversion_read_file <-> OBConversion::ReadFile
//! OBConversion_write_file <-> OBConversion::WriteFile
//! OBConversion_get_supported_input_format <-> OBConversion::GetSupportedInputFormat
//! OBConversion_get_supported_output_format <-> OBConversion::GetSupportedOutputFormat
//!
//! OBMol
//! -----
//! OBMol_new <-> new OBMol()
//! OBMol_num_atoms <-> OBMol::NumAtoms
//! OBMol_num_bonds <-> OBMol::NumBonds
//! OBMol_num_hvy_atoms <-> OBMol::NumHvyAtoms
//! OBMol_get_mol_wt <-> OBMol::GetMolWt
//!
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
        type OBConversion;

        // Debug
        fn print_global_instances();

        // OBConversion
        // fn OBConversion_smi_to_mol(smiles: &CxxString) -> UniquePtr<OBMol>;
        fn OBConversion_new() -> UniquePtr<OBConversion>;
        fn OBConversion_set_in_format(
            conv: &UniquePtr<OBConversion>,
            input_format: &CxxString,
        ) -> u32;
        fn OBConversion_set_out_format(
            conv: &UniquePtr<OBConversion>,
            output_format: &CxxString,
        ) -> u32;
        fn OBConversion_set_in_and_out_formats(
            conv: &UniquePtr<OBConversion>,
            input_format: &CxxString,
            output_format: &CxxString,
        ) -> u32;
        fn OBConversion_read_string(
            conv: &UniquePtr<OBConversion>,
            mol: &UniquePtr<OBMol>,
            input: &CxxString,
        ) -> u32;
        fn OBConversion_write_string(
            conv: &UniquePtr<OBConversion>,
            mol: &UniquePtr<OBMol>,
        ) -> String;
        fn OBConversion_read_file(
            conv: &UniquePtr<OBConversion>,
            mol: &UniquePtr<OBMol>,
            input_path: &CxxString,
        ) -> u32;
        fn OBConversion_write_file(
            conv: &UniquePtr<OBConversion>,
            mol: &UniquePtr<OBMol>,
            output_path: &CxxString,
        ) -> u32;
        fn OBConversion_get_supported_input_format() -> Vec<String>;
        fn OBConversion_get_supported_output_format() -> Vec<String>;

        // OBMol
        fn OBMol_new() -> UniquePtr<OBMol>;
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
