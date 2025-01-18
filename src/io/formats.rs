//! Openbabel InputFormat & OutputFormat 
//! 
//! Various molecule data formats, refer to OBMoleculeFormat and its child classes.
//! Not all supported formats from Openbabel are included. New formats can be added.
//! 

use strum_macros::{Display, EnumCount as EnumCountMacro, EnumString};

#[derive(Eq, PartialEq, Display, Debug, EnumCountMacro, EnumString)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum InputFormat {
    /// Canonical SMILES format
    can,
    /// Gaussian cube format
    cub,
    /// Gaussian cube format
    cube,
    /// DALTON output format
    dallog,
    /// DALTON input format
    dalmol,
    /// Gaussian formatted checkpoint file format
    fch,
    /// Gaussian formatted checkpoint file format
    fchk,
    /// Gaussian formatted checkpoint file format
    fck,
    /// Gaussian01 Output
    g01,
    /// Gaussian Output
    g7,
    /// Gaussian Output
    g14,
    /// Gaussian Output
    g90,
    /// Gaussian Output
    g92,
    /// Gaussian Output
    g96,
    /// Gaussian Output
    gal,
    /// Gaussian Z-Matrix Input
    gzmat,
    /// ORCA output format
    orca,
    /// SIESTA format
    siesta,
    /// SMILES format
    smi,
    /// SMILES format
    smiles,
    /// TurboMole Coordinate format
    tmol,
    /// XYZ cartesian coordinates format
    xyz,
    /// SDF format
    sd,
    sdf,
    /// PDB format
    pdb,
}

#[derive(Eq, PartialEq, Display, Debug, EnumString)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum OutputFormat {
    /// Gaussian Input
    com,
    /// Gaussian cube format
    cub,
    /// Gaussian cube format
    cube,
    /// DALTON input format
    dalmol,
    /// SMILES FIX format
    fix,
    /// Gaussian Input
    gau,
    /// Gaussian Input
    gjc,
    /// Gaussian Input
    gjf,
    /// Gaussian Z-Matrix Input
    gzmat,
    /// ORCA input format
    orcainp,
    /// SMILES format
    smi,
    /// SMILES format
    smiles,
    /// TurboMole Coordinate format
    tmol,
    /// XYZ cartesian coordinates format
    xyz,
    /// SDF format
    sd,
    sdf,
}
