//! OpenBabel ForceField
//!
//! Enums to select the force field and the method of optimization

#[derive(Eq, PartialEq)]
pub enum ForceField {
    /// No force field has been selected
    None,

    /// Generalized Amber Force Field
    GAFF,

    /// Ghemical
    Ghemical,

    /// Merck Molecular Force Field
    MMFF94,

    /// Merck Molecular Force Field (static variant)
    MMFF94s,

    /// Universal Force Field
    UFF,
}
impl ForceField {
    pub fn as_str(&self) -> String {
        let ff_name = match self {
            ForceField::GAFF => "gaff",
            ForceField::Ghemical => "ghemical",
            ForceField::MMFF94 => "mmff94",
            ForceField::MMFF94s => "mmff94s",
            ForceField::UFF => "uff",
            ForceField::None => "no ff selected",
        };
        format!("{}", ff_name)
    }
}

pub enum Method {
    ConjugateGradient,
    SteepestDescent,
}
