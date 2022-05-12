/**********************************************************************
finger2.cpp: split from openbabel finger2.cpp 
***********************************************************************/

#include <finger2.h>

using namespace std;
namespace OpenBabel
{
/*! class fingerprint2
Similar to Fabien Fontain's fingerprint class, with a slightly improved
algorithm, but re-written using STL which makes it shorter.
*/

bool fingerprint2::GetFingerprint(OBBase* pOb, vector<unsigned int>&fp, int nbits)
{
	OBMol* pmol = dynamic_cast<OBMol*>(pOb);
	if(!pmol) return false;
	fp.resize(1024/Getbitsperint());
	fragset.clear();//needed because now only one instance of fp class
	ringset.clear();
 
	//identify fragments starting at every atom
	OBAtom *patom;
	vector<OBNodeBase*>::iterator i;
	for (patom = pmol->BeginAtom(i);patom;patom = pmol->NextAtom(i))
	{
		if(patom->GetAtomicNum() == OBElements::Hydrogen) continue;
		vector<int> curfrag;
		vector<int> levels(pmol->NumAtoms());
		getFragments(levels, curfrag, 1, patom, nullptr);
	}

//	TRACE("%s %d frags before; ",pmol->GetTitle(),fragset.size());

	//Ensure that each chemically identical fragment is present only in a single
	DoRings();
	DoReverses();

	SetItr itr;
  _ss.str("");
	for(itr=fragset.begin();itr!=fragset.end();++itr)
	{
		//Use hash of fragment to set a bit in the fingerprint
		int hash = CalcHash(*itr);
		SetBit(fp,hash);
		if(!(Flags() & FPT_NOINFO))
      PrintFpt(*itr,hash);
	}
	if(nbits)
		Fold(fp, nbits);

//	TRACE("%d after\n",fragset.size());
	return true;
}

//////////////////////////////////////////////////////////
void fingerprint2::getFragments(vector<int> levels, vector<int> curfrag,
					int level, OBAtom* patom, OBBond* pbond)
{
	//Recursive routine to analyse schemical structure and populate fragset and ringset
	//Hydrogens,charges(except dative bonds), spinMultiplicity ignored
	const int Max_Fragment_Size = 7;
	int bo=0;
	if(pbond)
	{
		bo = pbond->IsAromatic() ? 5 : pbond->GetBondOrder();

//		OBAtom* pprevat = pbond->GetNbrAtom(patom);
//		if(patom->GetFormalCharge() && (patom->GetFormalCharge() == -pprevat->GetFormalCharge()))
//			++bo; //coordinate (dative) bond eg C[N+]([O-])=O is seen as CN(=O)=O
	}
	curfrag.push_back(bo);
	curfrag.push_back(patom->GetAtomicNum());
	levels[patom->GetIdx()-1] = level;

	vector<OBBond*>::iterator itr;
	OBBond *pnewbond;
//	PrintFpt(curfrag,(int)patom);
	for (pnewbond = patom->BeginBond(itr);pnewbond;pnewbond = patom->NextBond(itr))
	{
		if(pnewbond==pbond) continue; //don't retrace steps
		OBAtom* pnxtat = pnewbond->GetNbrAtom(patom);
		if(pnxtat->GetAtomicNum() == OBElements::Hydrogen) continue;

		int atlevel = levels[pnxtat->GetIdx()-1];
		if(atlevel) //ring
		{
			if(atlevel==1)
			{
				//If complete ring (last bond is back to starting atom) add bond at front
				//and save in ringset
				curfrag[0] = pnewbond->IsAromatic() ? 5 : pnewbond->GetBondOrder();
				ringset.insert(curfrag);
 				curfrag[0] = 0;
			}
		}
		else //no ring
		{
			if(level<Max_Fragment_Size)
			{
//				TRACE("level=%d size=%d %p frag[0]=%p\n",level, curfrag.size(),&curfrag, &(curfrag[0]));
				//Do the next atom; levels, curfrag are passed by value and hence copied
				getFragments(levels, curfrag, level+1, pnxtat, pnewbond);
			}
		}
	}

	//do not save C,N,O single atom fragments
	if(curfrag[0]==0 &&
		(level>1 || patom->GetAtomicNum()>8  || patom->GetAtomicNum()<6))
	{
		fragset.insert(curfrag); //curfrag ignored if an identical fragment already present
//		PrintFpt(curfrag,level);
	}
}

///////////////////////////////////////////////////
void fingerprint2::DoReverses()
{
	SetItr itr;
	for(itr=fragset.begin();itr!=fragset.end();)
	{
		//Reverse the order of the atoms, add the smallest fragment and remove the larger
		SetItr titr = itr++; //Ensure have valid next iterator in case current one is erased
		vector<int> t1(*titr); //temporary copy
		reverse(t1.begin()+1, t1.end()); //(leave 0 at front alone)
		if(t1!=*titr)
		{
			//Add the larger fragment and delete the smaller
			if(t1>*titr)
			{
				fragset.erase(titr);
				fragset.insert(t1);
			}
			else
				fragset.erase(t1);
		}
	}
}
///////////////////////////////////////////////////
void fingerprint2::DoRings()
{
	//For each complete ring fragment, find its largest chemically identical representation
	//by rotating and reversing, and insert into the main set of fragments
	SetItr itr;
	for(itr=ringset.begin();itr!=ringset.end();++itr)
	{
		vector<int> t1(*itr); //temporary copy
		vector<int> maxring(*itr); //the current largest vector
		unsigned int i;
		for(i=0;i<t1.size()/2;++i)
		{
			//rotate atoms in ring
			rotate(t1.begin(),t1.begin()+2,t1.end());
			if(t1>maxring)
				maxring=t1;

			//reverse the direction around ring
			vector<int> t2(t1);
			reverse(t2.begin()+1, t2.end());
			if(t2>maxring)
				maxring=t2;
		}
		fragset.insert(maxring);
		//PrintFpt(maxring,0);
	}
}

//////////////////////////////////////////////////////////
unsigned int fingerprint2::CalcHash(const vector<int>& frag)
{
	//Something like... whole of fragment treated as a binary number modulus 1021
	const int MODINT = 108; //2^32 % 1021
	unsigned int hash=0;
	for(unsigned i=0;i<frag.size();++i)
		hash= (hash*MODINT + (frag[i] % 1021)) % 1021;
	return hash;
}

void fingerprint2::PrintFpt(const vector<int>& f, int hash)
{
	unsigned int i;
	for(i=0;i<f.size();++i)
    _ss  << f[i] << " ";
  _ss << "<" << hash << ">" << endl;
}

} //namespace OpenBabel

//! \file finger2.cpp
//! \brief fingerprint2 definition and implementation
