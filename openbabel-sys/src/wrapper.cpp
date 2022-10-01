#include <sstream>
#include <vector>
#include <openbabel/fingerprint.h>
#include <openbabel/oberror.h>
#include <openbabel/obconversion.h>
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

std::unique_ptr<OBConversion> OBConversion_new() { return std::unique_ptr<OBConversion>(new OBConversion()); }

unsigned int OBConversion_set_in_format(const std::unique_ptr<OBConversion> & pConv, const std::string &input_format) {
    OBFormat* pFormat = OBConversion::FindFormat(input_format);

    if (!pConv.get()->SetInFormat(pFormat)) {
	std::stringstream errorMsg;
        errorMsg << "OBConversion::SetInFormat(" << input_format << ")" << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
        return 1;
    }

    return 0;
}

unsigned int OBConversion_set_out_format(const std::unique_ptr<OBConversion> & pConv, const std::string &output_format) {
    OBFormat* pFormat = OBConversion::FindFormat(output_format);

    if (!pConv.get()->SetOutFormat(pFormat)) {
	std::stringstream errorMsg;
        errorMsg << "OBConversion::SetOutFormat(" << output_format << ")" << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
        return 1;
    }

    return 0;
}

unsigned int OBConversion_set_in_and_out_formats(const std::unique_ptr<OBConversion> & pConv, const std::string &input_format, const std::string &output_format) {
    OBFormat* pInFormat = OBConversion::FindFormat(input_format);
    OBFormat* pOutFormat = OBConversion::FindFormat(output_format);

    if (!pConv.get()->SetInAndOutFormats(pInFormat, pOutFormat)) {
	std::stringstream errorMsg;
        errorMsg << "OBConversion::SetInAndOutFormats(" << output_format << ")" << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
        return 1;
    }

    return 0;
}

unsigned int OBConversion_read_string(const std::unique_ptr<OBConversion> & pConv, const std::unique_ptr<OBMol> & pMol, const std::string &input) {
    if (!pConv.get()->ReadString(pMol.get(), input.c_str())) {
	std::stringstream errorMsg;
        errorMsg << "OBConversion::ReadString error" << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
	return 1;
    }
    return 0;
}

rust::String OBConversion_write_string(const std::unique_ptr<OBConversion> & pConv, const std::unique_ptr<OBMol> & pMol) {
    return pConv.get()->WriteString(pMol.get());
}

unsigned int OBConversion_read_file(const std::unique_ptr<OBConversion> & pConv, const std::unique_ptr<OBMol> & pMol, const std::string &input_path) {
    if (!pConv.get()->ReadFile(pMol.get(), input_path)) {
	std::stringstream errorMsg;
        errorMsg << "OBConversion::ReadFile error" << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
	return 1;
    }
    return 0;
}

unsigned int OBConversion_write_file(const std::unique_ptr<OBConversion> & pConv, const std::unique_ptr<OBMol> & pMol, const std::string &output_path) {
    if (!pConv.get()->WriteFile(pMol.get(), output_path)) {
        std::stringstream errorMsg;
        errorMsg << "OBConversion::WriteFile error" << std::endl;
        obErrorLog.ThrowError(__FUNCTION__, errorMsg.str(), obError);
	return 1;
    }
    return 0;
}

rust::Vec<rust::String> OBConversion_get_supported_input_format() {
    OBConversion obconv = OBConversion();
    rust::Vec<rust::String> result {};
    for (auto element : obconv.GetSupportedInputFormat()) {
	result.push_back(element);
    }
    return result;
}

rust::Vec<rust::String> OBConversion_get_supported_output_format() {
    OBConversion obconv = OBConversion();
    rust::Vec<rust::String> result {};
    for (auto element : obconv.GetSupportedOutputFormat()) {
	result.push_back(element);
    }
    return result;
}

// OBConversion - End


// OBMol
std::unique_ptr<OBMol> OBMol_new() { return std::unique_ptr<OBMol>(new OBMol()); }
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