use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetQIMagPercentResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetQIMagPercentResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let result = match parts[0].parse() {
            Ok(result) => result,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(SetQIMagPercentResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// TO USE THIS COMMAND, AUTO-GAIN MUST BE DISABLED FIRST
/// This command sets the magnitude setting of the IQ modulator, which regulates the ISC board's power output.
/// The higher the value, the higher the power output.
/// Remark: Under normal conditions, both the VGA and the IQ modulator are used to regulate the power output of the ISC board,
/// thus the actual power output is a combination of both.
pub struct SetQIMagPercent {
    channel: u8,
    /// The desired magnitude of the IQ modulator in percent (%).
    magnitude: u8,
}

impl Into<String> for SetQIMagPercent {
    fn into(self) -> String {
        format!("$MCS,{},{}", self.channel, self.magnitude)
    }
}

impl SetQIMagPercent {
    /// Magnitude in percent (%) in range from 0-100
    pub fn new(self, channel: u8, magnitude: u8) -> Self {
        Self { channel, magnitude }
    }
}

impl Default for SetQIMagPercent {
    fn default() -> Self {
        Self {
            channel: 1,
            magnitude: 75,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetQIMagPercentResponse {
    /// The uptime in seconds.
    pub magnitude: u8,
}

impl TryFrom<String> for GetQIMagPercentResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let magnitude = match parts[0].parse() {
            Ok(magnitude) => magnitude,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetQIMagPercentResponse { magnitude })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetQIMagPercent {
    channel: u8,
}

impl Into<String> for GetQIMagPercent {
    fn into(self) -> String {
        format!("$MCG,{}", self.channel)
    }
}

impl GetQIMagPercent {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetQIMagPercent {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
