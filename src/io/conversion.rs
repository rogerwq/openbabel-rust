//! Openbabel Conversion 
//! 
//! Wrapper of ob::OBConversion
//! 

use ob_rs::ob;
use super::formats::OutputFormat;

pub struct Conversion {
    pub ob_conv: cxx::UniquePtr<ob::OBConversion>, 
}

impl Conversion {
    pub fn new() -> Self {
        let ob_conv = ob::OBConversion_new();
        Self { ob_conv }
    }

    pub fn set_output_format(&self, output_format: &OutputFormat) {
        cxx::let_cxx_string!(output_format_cxx = output_format.to_string());
        ob::OBConversion_set_out_format(&self.ob_conv, &output_format_cxx);
    }
}