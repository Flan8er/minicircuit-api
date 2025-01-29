use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetRFOutputResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetRFOutputResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetRFOutputResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Turns RF output of the ISC board ON or OFF.
///
/// Board is turned off by default.
pub struct SetRFOutput {
    /// Channel identification number.
    pub channel: Channel,
    /// Desired setting of the RF output.
    ///
    /// True = ON
    ///
    /// False = OFF (default)
    pub enabled: bool,
}

impl Into<String> for SetRFOutput {
    fn into(self) -> String {
        let numeric_value = match self.enabled {
            true => 1,
            false => 0,
        };
        format!("$ECS,{},{}", self.channel, numeric_value)
    }
}

impl SetRFOutput {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(self, channel: Channel, enabled: bool) -> Self {
        Self { channel, enabled }
    }
}

impl Default for SetRFOutput {
    /// Returns the default handler to call the command.
    ///
    /// By default, output is disabled.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            enabled: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetRFOutputResponse {
    /// State of the ISC board's output.
    pub enabled: bool,
}

impl TryFrom<String> for GetRFOutputResponse {
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

        let enabled: bool = match parts[2].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetRFOutputResponse { enabled })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the enable state of the ISC board's RF output.
///
/// Enable state can be set with `SetRFOutput`, but there are also many status
/// conditions that turn RF output OFF for safety reasons. Check `GetStatus` for details.
pub struct GetRFOutput {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetRFOutput {
    fn into(self) -> String {
        format!("$ECG,{}", self.channel)
    }
}

impl GetRFOutput {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetRFOutput {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
