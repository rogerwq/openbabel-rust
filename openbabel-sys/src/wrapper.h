#pragma once
#include <memory>
#include "rust/cxx.h"
#include <openbabel/mol.h>
#include <openbabel/parsmart.h>
#include <openbabel/obconversion.h>

namespace OpenBabel {
    class OBMol;
    class OBSmartsPattern;
    class OBConversion;

    // Debug
    void print_global_instances();

    // OBConversion
    // std::unique_ptr<OBMol> OBConversion_smi_to_mol(const std::string &smiles);

    // OBMol
    std::unique_ptr<OBMol> OBMol_from_smiles(const std::string &smiles);
    unsigned int OBMol_num_atoms(const std::unique_ptr<OBMol> & pMol);
    unsigned int OBMol_num_bonds(const std::unique_ptr<OBMol> & pMol);
    unsigned int OBMol_num_hvy_atoms(const std::unique_ptr<OBMol> & pMol);
    double OBMol_get_mol_wt(const std::unique_ptr<OBMol> & pMol);

    // OBFingerprint
    typedef std::vector<unsigned int> FPData;
<<<<<<< HEAD
    std::unique_ptr<FPData> OBFingerprint_get_fingerprint(const std::string &fp_thread_name, const std::unique_ptr<OBMol> & pMol, u_int32_t nbits);
=======
    std::unique_ptr<FPData> OBFingerprint_get_fingerprint(const std::string &fp_name, const std::unique_ptr<OBMol> & pMol, u_int32_t nbits);
>>>>>>> dev
    // std::unique_ptr<FPData> OBFingerprint_get_fingerprint_in_batch(const std::string &fp_thread_name, const rust::Vec<rust::String> & smiles_vec, u_int32_t nbits);
    // deprecated: slow performance, root cause to be identified

    // OBSmartsPattern
    std::unique_ptr<OBSmartsPattern> OBSmartsPattern_from_smarts(const std::string &smarts);
    unsigned int OBSmartsPattern_num_atoms(const std::unique_ptr<OBSmartsPattern> & pSP);
    unsigned int OBSmartsPattern_num_bonds(const std::unique_ptr<OBSmartsPattern> & pSP);
    unsigned int OBSmartsPattern_num_matches(const std::unique_ptr<OBSmartsPattern> & pSP);
    std::unique_ptr<std::vector<int>> OBSmartsPattern_match(const std::unique_ptr<OBSmartsPattern> & pSP, const std::unique_ptr<OBMol> &pMol);
}