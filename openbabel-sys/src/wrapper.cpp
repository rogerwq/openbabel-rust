#include <sstream>
#include <vector>
#include <thread>
#include <openbabel/fingerprint.h>
#include "wrapper.h"

namespace OpenBabel {

// For Debug Purpose

void print_global_instances() {
    // std::cout << "theSMIFormat: " << &theSMIFormat << std::endl;
    std::cout << "FP2: " << OBFingerprint::FindFingerprint("FP2") << std::endl;
    std::cout << "FP3: " << OBFingerprint::FindFingerprint("FP3") << std::endl;
    std::cout << "FP4: " << OBFingerprint::FindFingerprint("FP4") << std::endl;
    std::cout << "FP2 thread 0: " << OBFingerprint::FindFingerprint("FP2_thread_0") << std::endl;
    std::cout << "FP3 thread 0: " << OBFingerprint::FindFingerprint("FP3_thread_0") << std::endl;
    std::cout << "FP4 thread 0: " << OBFingerprint::FindFingerprint("FP4_thread_0") << std::endl;
    static int a = 0;
    std::cout << "staic a: " << &a << std::endl;
    // std::cout << "Class OBFingerprint: " << &(OBFingerprint::Map) << std::endl;
}

// Debug - End

// OBConversion 

std::unique_ptr<OBMol> OBConversion_smi_to_mol(const std::string &smiles) {
    std::unique_ptr<OBMol> pMol(new OBMol());
    std::stringstream ss(smiles);
    OBConversion conv(&ss);
    if(conv.SetInFormat("smi") && conv.Read(pMol.get())) {
        return pMol;
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

    if (!pFP || !pFP->GetFingerprint(pMol.get(), fps, nbits)) {
        fps.resize(0);
    }

    return std::make_unique<FPData>(std::move(fps));
}

std::unique_ptr<FPData> OBFingerprint_get_fingerprint_in_batch(const std::string &fp_thread_name, const rust::Vec<rust::String> & smiles_vec, u_int32_t nbits) {
    FPData fps, results;
    results.resize(0);

    OBFingerprint* pFP = OBFingerprint::FindFingerprint(fp_thread_name.c_str());
    OBConversion conv;
    OBMol* pMol = new OBMol();
    u_int32_t fp_bits = (nbits < pFP->Getbitsperint()) ? nbits : pFP->Getbitsperint();

    if (pFP && conv.SetInFormat("smi")) {
        for (std::size_t i = 0; i < smiles_vec.size(); ++i) {
            fps.resize(0);

            if (conv.ReadString(pMol, std::string(smiles_vec[i]))) {
                if(!pFP->GetFingerprint(pMol, fps, nbits)) {
                     fps.resize(fp_bits); // error of generating fingerprint
                }
            } else { // If the conversion from SMILES to mol is not successful, set the fingerprint data to ZERO.
                fps.resize(fp_bits);
            }
            results.insert(results.end(), std::make_move_iterator(fps.begin()), std::make_move_iterator(fps.end()));
        }
    }

    if (pMol) free(pMol);

    return std::make_unique<FPData>(std::move(results));
}

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