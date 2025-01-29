use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Amperes, Channel},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPACurrentResponse {
    /// DC current readings of the ISC in Amps.
    pub current: Amperes,
}

impl TryFrom<String> for GetPACurrentResponse {
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

        let current: Amperes = match parts[2].trim().parse::<f32>() {
            Ok(value) => Amperes::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPACurrentResponse { current })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the DC current reading of the ISC in Amps.
pub struct GetPACurrent {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPACurrent {
    fn into(self) -> String {
        format!("$PIG,{}", self.channel)
    }
}

impl GetPACurrent {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPACurrent {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
