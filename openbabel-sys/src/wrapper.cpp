#include <sstream>
#include <vector>
#include <openbabel/fingerprint.h>
#include <openbabel/oberror.h>
#include "wrapper.h"
#include "smilesformat.h"

namespace OpenBabel {

// OBConversion 

std::unique_ptr<OBMol> OBConversion_smi_to_mol(const std::string &smiles) {
    OBSmilesParser ob_sp = OBSmilesParser();
    OBMol mol = OBMol();
    if (ob_sp.SmiToMol(mol, smiles)) {
        return std::make_unique<OBMol>(std::move(mol));
    } else {
        return std::unique_ptr<OBMol>(nullptr);
    }
}

// OBConversion - End


// OBMol

unsigned int OBMol_num_atoms(const std::unique_ptr<OBMol> & pMol) { return pMol->NumAtoms(); }
unsigned int OBMol_num_bonds(const std::unique_ptr<OBMol> & pMol) { return pMol->NumBonds(); }
unsigned int OBMol_num_hvy_atoms(const std::unique_ptr<OBMol> & pMol) { return pMol->NumHvyAtoms(); }
double OBMol_get_mol_wt(const std::unique_ptr<OBMol> & pMol) { return pMol->GetMolWt(); }

// OBMol End

// OBFingerprint

std::unique_ptr<FPData> OBFingerprint_get_fingerprint(const std::string &fp_thread_name, const std::unique_ptr<OBMol> & pMol, u_int32_t nbits) {
    FPData fps;
    OBFingerprint* pFP = OBFingerprint::FindFingerprint(fp_thread_name.c_str());

    stringstream errorMsg;
    if (!pFP) {
        errorMsg << "Cannot find fingerprint " << fp_thread_name << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
        fps.resize(nbits / 32);
        std::fill(fps.begin(), fps.end(), 0);
    } else {
        if(!pFP->GetFingerprint(pMol.get(), fps, nbits)) {
            errorMsg << "Error on generating fingerprint " << fp_thread_name << std::endl;
            obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
            std::fill(fps.begin(), fps.end(), 0);
        }
    }

    return std::make_unique<FPData>(std::move(fps));
}

// std::unique_ptr<FPData> OBFingerprint_get_fingerprint_in_batch(const std::string &fp_thread_name, const rust::Vec<rust::String> & smiles_vec, u_int32_t nbits) {
//     FPData fps, results;
//     results.resize(0);
//     stringstream errorMsg;

//     OBFingerprint* pFP = OBFingerprint::FindFingerprint(fp_thread_name.c_str());
//     if (!pFP) {
//         errorMsg << "Cannot find fingerprint " << fp_thread_name << std::endl;
//         obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
//         return std::make_unique<FPData>(std::move(results));
//     } 

//     OBSmilesParser ob_sp = OBSmilesParser();
//     OBMol mol = OBMol();
//     for (std::size_t i = 0; i < smiles_vec.size(); ++i) {
//         fps.resize(0);
//         if (ob_sp.SmiToMol(mol, std::string(smiles_vec[i]))) {
//             if (!pFP->GetFingerprint(&mol, fps, nbits)) {
//                 errorMsg << "Error on generating fingerprint " << fp_thread_name << std::endl;
//                 obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
//                 std::fill(fps.begin(), fps.end(), 0);
//             } 
//         } else { 
//             errorMsg << "Error on SMILES parsing " << smiles_vec[i] << std::endl;
//             obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
//             fps.resize(nbits / 32);
//             std::fill(fps.begin(), fps.end(), 0);
//         }
//         results.insert(results.end(), std::make_move_iterator(fps.begin()), std::make_move_iterator(fps.end()));
//     }

//     return std::make_unique<FPData>(std::move(results));
// }

// OBFingerprint - End


// OBSmartsPattern

std::unique_ptr<OBSmartsPattern> OBSmartsPattern_new(const std::string &smarts) {
    std::unique_ptr<OBSmartsPattern> pSP(new OBSmartsPattern());
    pSP->Init(smarts);
    // return std::make_unique<OBSmartsPattern>(sp);
    return pSP;
}

unsigned int OBSmartsPattern_num_atoms(const std::unique_ptr<OBSmartsPattern> & pSP) { return pSP->NumAtoms(); }
unsigned int OBSmartsPattern_num_bonds(const std::unique_ptr<OBSmartsPattern> & pSP) { return pSP->NumBonds(); }
unsigned int OBSmartsPattern_num_matches(const std::unique_ptr<OBSmartsPattern> & pSP) { return pSP->NumMatches(); }

std::unique_ptr<std::vector<int>> OBSmartsPattern_match(const std::unique_ptr<OBSmartsPattern> & pSP, const std::unique_ptr<OBMol> & pMol) {
    pSP->Match(*pMol);
    // CxxVector does not support nested C++ vector (std::vector<std::vector>)
    std::vector<int> result {};
    for (std::vector<std::vector<int>>::iterator i = pSP->GetMapList().begin(); i != pSP->GetMapList().end(); ++i) {
        result.insert(result.end(), i->begin(), i->end());
    }

    return std::make_unique<std::vector<int>>(std::move(result));
}

// OBSmartsPattern - End


} // namespace OpenBabel