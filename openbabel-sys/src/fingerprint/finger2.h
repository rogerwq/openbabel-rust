/**********************************************************************
finger2.h: split from openbabel finger2.cpp 
***********************************************************************/

#include <openbabel/babelconfig.h>
#include <openbabel/oberror.h>
#include <openbabel/mol.h>
#include <openbabel/atom.h>
#include <openbabel/bond.h>
#include <openbabel/fingerprint.h>
#include <set>
#include <vector>
#include <algorithm>
#include <openbabel/elements.h>

using namespace std;
namespace OpenBabel
{
/// \brief Fingerprint based on linear fragments up to 7 atoms ID="FP2"
class fingerprint2 : public OBFingerprint
{
public:
	fingerprint2(const char* ID, bool IsDefault=false)
		: OBFingerprint(ID, IsDefault), _flags(0){};

	virtual const char* Description()
	{ return "Indexes linear fragments up to 7 atoms."
    "\n1021 bits.\n"
    "Similar to Daylight fingerprints\n"
    "A molecule structure is analysed to identify linear fragments of length\n"
    "from one to Max_Fragment_Size = 7 atoms but single atom fragments of C,N,and O\n"
    "are ignored. A fragment is terminated when the atoms form a ring.\n"
    "For each of these fragments the atoms, bonding and whether they constitute\n"
    "a complete ring is recorded and saved in a std::set, so that there is\n"
    "only one of each fragment type. Chemically identical versions, i.e. ones with\n"
    "the atoms listed in reverse order and rings listed starting at different\n"
    "atoms, are identified and only a single canonical fragment is retained\n"
    "Each remaining fragment is assigned a hash number from 0 to 1020 which is\n"
    "used to set a bit in a 1024 bit vector.\n"
    "For further details see:\n"
    "http://baoilleach.blogspot.co.uk/2012/01/visualising-fragments-in-path-based.html \n"  
  ;}

	//Calculates the fingerprint
	virtual bool GetFingerprint(OBBase* pOb, vector<unsigned int>&fp, int nbits=0);

  /// \returns fragment info unless SetFlags(OBFingerprint::FPT_NOINFO) has been called before GetFingerprint() called. 
  /** Structure of a fragment (vector<int>)
   For a complete ring: last atom bonded to first atom
      bo(0)(n), atno(1), bo(1)(2), atno(2), bo(2)(3),...atno(n)
   For the rest, even when stopped by encountering atoms already visited
         0    , atno(1), bo(1)(2), atno(2), bo(2)(3),...atno(n)
  **/
virtual std::string DescribeBits(const std::  vector<unsigned int> fp, bool bSet=true)
  { return _ss.str(); }

  virtual unsigned int Flags() { return _flags;};
  virtual void SetFlags(unsigned int f){ _flags=f; }

private:
	typedef std::set<std::vector<int> > Fset;
	typedef std::set<std::vector<int> >::iterator SetItr;

	void getFragments(std::vector<int> levels, std::vector<int> curfrag,
			int level, OBAtom* patom, OBBond* pbond);
	void DoReverses();
	void DoRings();

	unsigned int CalcHash(const std::vector<int>& frag);
	void PrintFpt(const std::vector<int>& f, int hash=0);

	Fset fragset;
	Fset ringset;
  stringstream _ss;
  unsigned int _flags;

};
}