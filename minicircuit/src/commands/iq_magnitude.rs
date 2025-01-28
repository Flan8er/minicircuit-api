use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Percentage},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetQIMagPercentResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetQIMagPercentResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetQIMagPercentResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// TO USE THIS COMMAND, AUTO-GAIN MUST BE DISABLED FIRST (command not documented)
///
/// This command sets the magnitude setting of the IQ modulator, which regulates the ISC board's power output.
/// The higher the value, the higher the power output.
///
/// Remark: Under normal conditions, both the VGA and the IQ modulator are used to regulate the power output of the ISC board,
/// thus the actual power output is a combination of both.
pub struct SetQIMagPercent {
    /// Channel identification number.
    pub channel: Channel,
    /// The desired magnitude of the IQ modulator in percent (%).
    pub magnitude: Percentage,
}

impl Into<String> for SetQIMagPercent {
    fn into(self) -> String {
        format!("$MCS,{},{}", self.channel, self.magnitude)
    }
}

impl SetQIMagPercent {
    /// Magnitude in percent (%) in range from 0-100
    pub fn new(self, channel: Channel, magnitude: Percentage) -> Self {
        Self { channel, magnitude }
    }
}

impl Default for SetQIMagPercent {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            magnitude: Percentage::new(75),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetQIMagPercentResponse {
    /// The current magnitude configuration of the IQ modulator in percent.
    pub magnitude: Percentage,
}

impl TryFrom<String> for GetQIMagPercentResponse {
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

        let magnitude: Percentage = match parts[2].trim().parse::<u8>() {
            Ok(value) => Percentage::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetQIMagPercentResponse { magnitude })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Gets the magnitude of the IQ modulator.
pub struct GetQIMagPercent {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetQIMagPercent {
    fn into(self) -> String {
        format!("$MCG,{}", self.channel)
    }
}

impl GetQIMagPercent {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetQIMagPercent {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
