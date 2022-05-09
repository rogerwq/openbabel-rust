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

    // OBConversion
    std::unique_ptr<OBMol> OBConversion_smi_to_mol(const std::string &smiles);

    // OBMol
    unsigned int OBMol_num_atoms(const std::unique_ptr<OBMol> & pMol);
    unsigned int OBMol_num_bonds(const std::unique_ptr<OBMol> & pMol);
    unsigned int OBMol_num_hvy_atoms(const std::unique_ptr<OBMol> & pMol);
    double OBMol_get_mol_wt(const std::unique_ptr<OBMol> & pMol);

    // OBFingerprint
    std::unique_ptr<std::vector<unsigned int>> OBFingerprint_get_fingerprint(const std::string &fp_name, const std::unique_ptr<OBMol> & pMol, u_int32_t nbits);
    std::unique_ptr<std::vector<unsigned int>> OBFingerprint_get_fingerprint_in_batch(const std::string &fp_name, const std::vector<std::unique_ptr<OBMol>> & pMol, u_int32_t nbits);

    // OBSmartsPattern
    std::unique_ptr<OBSmartsPattern> OBSmartsPattern_new(const std::string &smarts);
    unsigned int OBSmartsPattern_num_atoms(const std::unique_ptr<OBSmartsPattern> & pSP);
    unsigned int OBSmartsPattern_num_bonds(const std::unique_ptr<OBSmartsPattern> & pSP);
    unsigned int OBSmartsPattern_num_matches(const std::unique_ptr<OBSmartsPattern> & pSP);
    std::unique_ptr<std::vector<int>> OBSmartsPattern_match(const std::unique_ptr<OBSmartsPattern> & pSP, const std::unique_ptr<OBMol> &pMol);
}