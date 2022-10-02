//! OpenBabel OBConversion
//!
//! Methods for specifying input and output formats.
//! # Create a Formats for later use
//! ```
//! use openbabel::conversion::{Formats, InputFormat, OutputFormat};
//!
//! let mut mol = Formats::new(Some(InputFormat::xyz), Some(OutputFormat::fchk));
//! ```

use ob_rs::ob;
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
    /// Gaussian03 Output
    g03,
    /// Gaussian Output
    g09,
    /// Gaussian Output
    g16,
    /// Gaussian Output
    g92,
    /// Gaussian Output
    g94,
    /// Gaussian Output
    g98,
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
}

#[derive(Eq, PartialEq, Display, Debug, EnumString)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum OutputFormat {
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
    /// Gaussian Output
    g03,
    /// Gaussian Output
    g09,
    /// Gaussian Output
    g16,
    /// Gaussian Output
    g92,
    /// Gaussian Output
    g94,
    /// Gaussian Output
    g98,
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
}

pub struct Formats {
    pub input_format: Option<InputFormat>,
    pub output_format: Option<OutputFormat>,

    pub ob_conv: cxx::UniquePtr<ob::OBConversion>,
}

impl Formats {
    pub fn new(
        input_format: Option<InputFormat>,
        output_format: Option<OutputFormat>,
    ) -> Result<Self, ()> {
        let ob_conv = ob::OBConversion_new();
        let mut tmp_input = None;
        let mut tmp_output = None;

        let result_in = match input_format {
            Some(input_format) => {
                cxx::let_cxx_string!(input_format_cxx = input_format.to_string());
                tmp_input = Some(input_format);
                ob::OBConversion_set_in_format(&ob_conv, &input_format_cxx)
            }
            None => 0,
        };
        let result_out = match output_format {
            Some(output_format) => {
                cxx::let_cxx_string!(output_format_cxx = output_format.to_string());
                tmp_output = Some(output_format);
                ob::OBConversion_set_out_format(&ob_conv, &output_format_cxx)
            }
            None => 0,
        };

        if result_in + result_out == 0 {
            Ok(Self {
                input_format: tmp_input,
                output_format: tmp_output,
                ob_conv,
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test_mod_conversion {
    use super::*;

    #[test]
    fn test_creation() {
        let format_specifier = Formats::new(Some(InputFormat::xyz), Some(OutputFormat::tmol))
            .expect("unable to set formats");

        assert_eq!(format_specifier.input_format.unwrap(), InputFormat::xyz);
        assert_eq!(format_specifier.output_format.unwrap(), OutputFormat::tmol);
    }
}
