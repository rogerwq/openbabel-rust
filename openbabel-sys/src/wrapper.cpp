#include <sstream>
#include <vector>
#include "finger2.h"
#include "finger3.h"
#include "wrapper.h"

/*
References:
    http://openbabel.org/dev-api/group__substructure.shtml
    http://openbabel.org/dev-api/classOpenBabel_1_1OBSmartsPattern.shtml
    MNA Fingerprint
    http://openbabel.org/docs/dev/FileFormats/Multilevel_Neighborhoods_of_Atoms_(MNA).html#multilevel-neighborhoods-of-atoms-mna
*/

namespace OpenBabel {

// For Debug Purpose
void print_global_instances() {
    std::cout << "theSMIFormat: " << &theSMIFormat << std::endl;
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

// std::unique_ptr<OBFingerprint> OBFingerprint_find(const std::string &fp_name) {
/*
    This method try to find the pointer to the OBFingerprint and save it to the Rust Fingerprint object.
    It seems unfeasible due to the code structure of OpenBabel since
    (1) OBFingerprint is a abstract class derived from OBPlugin with a pure virtual function GetFingerprint;
    (2) No header files available for the concrete fingerprint classes such as fingerprintECFP etc.
*/
/*
    Method 1: use the OBFingerprint pointers managed by openbable as instances of OBPlugin
    The rust tests passed while at the end openbabel throw out an error "pointer being freed was not allocated"
    Guess: the ownership of OBFingerprint object is moved to rust fingerprint and deallcoated at the end of rust object lifecyle. Meanwhile openbabel keeps free the plugins at the end of the program thus leading to an error of freeing non-allocated pointer.
*/
    // std::unique_ptr<OBFingerprint> pFp(OBFingerprint::FindFingerprint(fp_name.c_str()));
    // return pFp;
/*
    Method 2: make a copy the OBFingerprint instance
    Since OBFingerprint is an abstract class, casting from OBPlugin to OBFinerprint fails. Casting to the concrete fingerprint class is impossible without the header files.
*/
//     OBPlugin* pPlugin = OBFingerprint::FindFingerprint(fp_name.c_str());
//     OBFingerprint* pFp = static_cast<OBFingerprint>(pPlugin);
//     return std::unique_ptr<OBFingerprint>(pFp);
// }

OBFingerprint* OBFingerprint_get_ptr(const std::string &fp_name) {
    if (fp_name == "FP2") {
        // return new fingerprint2("FP2_temp", false);
        return new OBFingerprint::
    } else if (fp_name == "FP3") {
        return new PatternFP("FP3_temp");
    } else if (fp_name == "FP4") {
        return new PatternFP("FP4_temp", "SMARTS_InteLigand.txt");
    } else {
        return nullptr; 
    }
}

// std::unique_ptr<std::vector<unsigned int>> OBFingerprint_get_fingerprint(const std::string &fp_name, const std::unique_ptr<OBMol> & pMol, u_int32_t nbits) {
//     /*
//         OpenBabel creates global variables for fingerprints and uses a pre-loaded plugin system to manage different types of fingerprint.
//         Memory error "pointer being freed was not allocated" occurs randomly. The root cause has not been confirmed.
//         Debug method:
//             1. lldb
//             2. create target "..."
//             3. b malloc_error_break
//             4. r
//     */
//     std::vector<unsigned int> fps;
//     OBFingerprint* pFp = OBFingerprint::FindFingerprint(fp_name.c_str());
//     if (!pFp->GetFingerprint(pMol.get(), fps, nbits)) {
//         fps.resize(0);
//     }

//     return std::make_unique<std::vector<unsigned int>>(std::move(fps));
// }

std::unique_ptr<FPData> OBFingerprint_get_fingerprint(const std::string &fp_name, const std::unique_ptr<OBMol> & pMol, u_int32_t nbits) {
    FPData fps;
    OBFingerprint* pFP = OBFingerprint_get_ptr(fp_name);

    if (pFP && !pFP->GetFingerprint(pMol.get(), fps, nbits)) {
        fps.resize(0);
    }

    if (pFP) free(pFP);

    return std::make_unique<FPData>(std::move(fps));
}

std::unique_ptr<FPData> OBFingerprint_get_fingerprint_in_batch(const std::string &fp_name, const rust::Vec<rust::String> & smiles_vec, u_int32_t nbits) {
    FPData fps, results;
    results.resize(0);

    OBFingerprint* pFP = OBFingerprint_get_ptr(fp_name);
    OBConversion conv;
    OBMol* pMol = new OBMol();

    if (pFP && conv.SetInFormat("smi")) {
        for (std::size_t i = 0; i < smiles_vec.size(); ++i) {
            fps.resize(0);

            // SetInStream(new stringstream(input), true);
            // Read(pOb);

            if (conv.ReadString(pMol, std::string(smiles_vec[i]))) {
                //if(!pFP->GetFingerprint(pMol, fps, nbits)) {
                     fps.resize(nbits / 32);
                // }
            } else { // If the conversion from SMILES to mol is not successful, set the fingerprint data to ZERO.
                fps.resize(nbits / 32);
            }
            results.insert(results.end(), std::make_move_iterator(fps.begin()), std::make_move_iterator(fps.end()));
        }
    }

    if (pFP) free(pFP);
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