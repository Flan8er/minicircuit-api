use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Volts},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAVoltageResponse {
    /// Measured DC voltage of the PA in Volts.
    pub voltage: Volts,
}

impl TryFrom<String> for GetPAVoltageResponse {
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
        if parts.len() != 3 {
            return Err(Self::Error::FailedParseResponse);
        }

        let voltage: Volts = match parts[2].trim().parse::<f32>() {
            Ok(value) => Volts::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAVoltageResponse { voltage })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the measured DC voltage of the PA in Volts.
pub struct GetPAVoltage {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAVoltage {
    fn into(self) -> String {
        format!("$PVG,{}", self.channel)
    }
}

impl GetPAVoltage {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAVoltage {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
