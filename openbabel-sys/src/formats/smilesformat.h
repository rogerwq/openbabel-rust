/**********************************************************************
Copyright (C) 2005-2007 by Craig A. James, eMolecules Inc.
Some portions Copyright (C) 1998-2001 by OpenEye Scientific Software, Inc.
Some portions Copyright (C) 2001-2008 by Geoffrey R. Hutchison
Some portions Copyright (C) 2004 by Chris Morley
Some portions Copyright (C) 2019 by NextMove Software.

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation version 2 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
***********************************************************************/

// This code uses the old OpenEye SMILES parser
// but replaces the SMILES export with Craig James canonical smiles
// (For regular SMILES, the canonical order is not computed and ignored)

#include <openbabel/babelconfig.h>
#include <openbabel/obmolecformat.h>

#include <openbabel/mol.h>
#include <openbabel/atom.h>
#include <openbabel/bond.h>
#include <openbabel/obiter.h>
#include <openbabel/elements.h>
#include <openbabel/generic.h>


#include <openbabel/stereo/tetrahedral.h>
#include <openbabel/stereo/cistrans.h>
#include <openbabel/stereo/squareplanar.h>
#include <openbabel/reactionfacade.h>
#include <openbabel/stereo/stereo.h>
#include <openbabel/obfunctions.h>
#include <openbabel/graphsym.h>
#include <openbabel/kekulize.h>
#include <openbabel/canon.h>

#include "smilesvalence.h"

#include <limits>
#include <iostream>
#include <cassert>
#include <string>

//#define DEBUG 1
#define IMPLICIT_CIS_RING_SIZE 8

using namespace std;

namespace OpenBabel {
      class OBSmilesParser
  {
    // simple structs to make code more readable

    // see _extbond
    struct ExternalBond
    {
      int digit;
      int prev;
      int order;
      char updown;
    };
    // see _rclose
    struct RingClosureBond
    {
      int digit;
      int prev;
      int order;
      char updown;
      int numConnections;
    };


    char _updown;
    int _order;
    int _prev;
    int _rxnrole;
    const char *_ptr;
    bool _preserve_aromaticity;
    vector<int>             _vprev;
    vector<RingClosureBond> _rclose;
    vector<ExternalBond>    _extbond;
    vector<int>             _path;
    vector<bool>            _avisit;
    vector<bool>            _bvisit;
    vector<int>             _hcount;
    vector<int> PosDouble; //for extension: lc atoms as conjugated double bonds

    struct StereoRingBond
    {
      vector<OBAtom*> atoms;
      vector<char> updown;
    };
    map<OBBond*, StereoRingBond> _stereorbond; // Remember info on the stereo ring closure bonds

    // stereochimistry
    bool chiralWatch; // set when a tetrahedral atom is read
    map<OBAtom*, OBTetrahedralStereo::Config*> _tetrahedralMap; // map of tetrahedral atoms and their data
    map<OBBond*, char> _upDownMap; // store the '/' & '\' as they occurred in smiles
    map<unsigned int, char> _chiralLonePair; // for atoms with potential chiral lone pairs, remember when the l.p. was encountered
    bool squarePlanarWatch; // set when a square planar atom is read
    map<OBAtom*, OBSquarePlanarStereo::Config*> _squarePlanarMap;

  public:

    OBSmilesParser(bool preserve_aromaticity=false): _preserve_aromaticity(preserve_aromaticity), _rxnrole(1) { }
    ~OBSmilesParser() { }

    bool SmiToMol(OBMol&,const string&);
    bool ParseSmiles(OBMol&, const string&);
    bool ParseSimple(OBMol&);
    bool ParseComplex(OBMol&);
    bool ParseRingBond(OBMol&);
    bool ParseExternalBond(OBMol&);
    bool CapExternalBonds(OBMol &mol);
    int NumConnections(OBAtom *, bool isImplicitRef=false);
    void CreateCisTrans(OBMol &mol);
    char SetRingClosureStereo(StereoRingBond rcstereo, OBBond* dbl_bond);
    void InsertTetrahedralRef(OBMol &mol, unsigned long id);
    void InsertSquarePlanarRef(OBMol &mol, unsigned long id);

    bool IsUp(OBBond*);
    bool IsDown(OBBond*);
  };

} // end namespace OpenBabel