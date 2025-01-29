use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Adc, Channel},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerADCResponse {
    /// The forward power ADC count from 0 to 4095.
    pub forward: Adc,
    /// The reflected power ADC count from 0 to 4095.
    pub reflected: Adc,
}

impl TryFrom<String> for GetPAPowerADCResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // First, check for errors in the response
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        // If there are no errors parse the response into struct components
        let parts: Vec<&str> = response.split(',').collect();

        // Ensure the input has the expected number of parts
        if parts.len() != 4 {
            return Err(Self::Error::FailedParseResponse);
        }

        let forward: Adc = match parts[2].trim().parse::<f32>() {
            Ok(value) => Adc::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflected: Adc = match parts[3].trim().parse::<f32>() {
            Ok(value) => Adc::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAPowerADCResponse { forward, reflected })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the forward and reflected power as ADC counts.
///
/// Depending on the PA Type, these ADC counts are either converted from the analog voltage inputs on the ISC board,
/// or from the ADCs on the ZHL-2425-250X+ (See `SetPAType`). If the source of the ADC count is the ISC board,
/// ADC measurements are averaged over 10 samples. Otherwise, a single sample is returned.
pub struct GetPAPowerADC {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAPowerADC {
    fn into(self) -> String {
        format!("$PAG,{}", self.channel)
    }
}

impl GetPAPowerADC {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerADC {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
