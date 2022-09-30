#include <sstream>
#include <vector>
#include <openbabel/fingerprint.h>
#include <openbabel/oberror.h>
#include <openbabel/obconversion.h>
#include <openbabel/forcefield.h>
#include "wrapper.h"

namespace OpenBabel {

// OBConversion 

// std::unique_ptr<OBMol> OBConversion_smi_to_mol(const std::string &smiles) {
//     OBSmilesParser ob_sp = OBSmilesParser();
//     OBMol mol = OBMol();
//     if (ob_sp.SmiToMol(mol, smiles)) {
//         return std::make_unique<OBMol>(std::move(mol));
//     } else {
//         return std::unique_ptr<OBMol>(nullptr);
//     }
// }

// OBConversion - End

// OBForceField
std::unique_ptr<OBForceField> OBForceField_find_forcefield(const std::string &ff_name) {
    OBForceField* raw_ff = OBForceField::FindForceField(ff_name.c_str());
    std::unique_ptr<OBForceField> p_ff(raw_ff->MakeNewInstance());

    if (!p_ff) {
	std::stringstream errorMsg;
	errorMsg << "OBForceField::FindForceField error" << std::endl;
	obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
    }

    return p_ff;
}

unsigned int OBForceField_setup(const std::unique_ptr<OBMol> & pMol, const std::unique_ptr<OBForceField> & pFF) {
    if (!pFF.get()->Setup(*pMol)) {
	std::stringstream errorMsg;
	errorMsg << "OBForceField->Setup() error" << std::endl;
	obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
	return 1;
    }
    return 0;
}

void OBForceField_conjugate_gradients(const std::unique_ptr<OBForceField> & pFF, u_int32_t steps, double econv) {
    pFF.get()->ConjugateGradients(steps, econv);
}

void OBForceField_conjugate_gradients_initialize(const std::unique_ptr<OBForceField> & pFF, u_int32_t steps, double econv) {
    pFF.get()->ConjugateGradientsInitialize(steps, econv);
}

bool OBForceField_conjugate_gradients_take_n_steps(const std::unique_ptr<OBForceField> & pFF, u_int32_t n) {
    return pFF.get()->ConjugateGradientsTakeNSteps(n);
}

void OBForceField_steepest_descent(const std::unique_ptr<OBForceField> & pFF, u_int32_t steps, double econv) {
    pFF.get()->SteepestDescent(steps, econv);
}

void OBForceField_steepest_descent_initialize(const std::unique_ptr<OBForceField> & pFF, u_int32_t steps, double econv) {
    pFF.get()->SteepestDescentInitialize(steps, econv);
}

bool OBForceField_steepest_descent_take_n_steps(const std::unique_ptr<OBForceField> & pFF, u_int32_t n) {
    return pFF.get()->SteepestDescentTakeNSteps(n);
}

double OBForceField_energy(const std::unique_ptr<OBForceField> & pFF) { return pFF.get()->Energy(); }
bool OBForceField_is_setup_needed(const std::unique_ptr<OBForceField> & pFF, const std::unique_ptr<OBMol> & pMol) { return pFF.get()->IsSetupNeeded(*pMol); }

// OBForceField End


// OBMol
std::unique_ptr<OBMol> OBMol_from_smiles(const std::string &smiles) {
    std::unique_ptr<OBMol> pMol(new OBMol());
    std::stringstream ss(smiles);
    OBConversion conv(&ss);

    if (!conv.SetInFormat("smi")) {
        std::stringstream errorMsg;
        errorMsg << "OBConversion::SetInFormat (\"smi\")  error" << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
    }

    if(!conv.Read(pMol.get())) {
        std::stringstream errorMsg;
        errorMsg << "OBConversion::Read error" << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
        return std::unique_ptr<OBMol>(nullptr);
    } 

    return pMol;
}

unsigned int OBMol_num_atoms(const std::unique_ptr<OBMol> & pMol) { return pMol->NumAtoms(); }
unsigned int OBMol_num_bonds(const std::unique_ptr<OBMol> & pMol) { return pMol->NumBonds(); }
unsigned int OBMol_num_hvy_atoms(const std::unique_ptr<OBMol> & pMol) { return pMol->NumHvyAtoms(); }
double OBMol_get_mol_wt(const std::unique_ptr<OBMol> & pMol) { return pMol->GetMolWt(); }

// OBMol End

// OBFingerprint

std::unique_ptr<FPData> OBFingerprint_get_fingerprint(const std::string &fp_name, const std::unique_ptr<OBMol> & pMol, u_int32_t nbits) {
    FPData fps;
    fps.resize(nbits / 32);
    OBFingerprint* pFP = OBFingerprint::FindFingerprint(fp_name.c_str());

    if (!pFP) {
        std::stringstream errorMsg;
        errorMsg << "Cannot find fingerprint " << fp_name << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
        std::fill(fps.begin(), fps.end(), 0);
    } else {
        if(!pFP->GetFingerprint(pMol.get(), fps, nbits)) {
            std::stringstream errorMsg;
            errorMsg << "Error on generating fingerprint " << fp_name << std::endl;
            obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
            std::fill(fps.begin(), fps.end(), 0);
        }
    }

    return std::make_unique<FPData>(std::move(fps));
}

// OBFingerprint - End


// OBSmartsPattern

std::unique_ptr<OBSmartsPattern> OBSmartsPattern_from_smarts(const std::string &smarts) {
    std::unique_ptr<OBSmartsPattern> pSP(new OBSmartsPattern());
    pSP->Init(smarts);
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