use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetRFOutputResponse {
    /// The result of the command (Ok/Err)
    pub result: String,
}

impl TryFrom<String> for SetRFOutputResponse {
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

        Ok(SetRFOutputResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetRFOutput {
    channel: u8,
    /// Desired setting of the RF output (on or off)
    enabled: bool,
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
    pub fn new(self, channel: u8, enabled: bool) -> Self {
        Self { channel, enabled }
    }
}

impl Default for SetRFOutput {
    fn default() -> Self {
        Self {
            channel: 1,
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
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        // let result = match parts[0].parse() {
        //     Ok(result) => result,
        //     Err(_) => return Err(MWError::FailedParseResponse),
        // };

        // Hardcoding this for now but go back and change later
        Ok(GetRFOutputResponse { enabled: true })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetRFOutput {
    channel: u8,
}

impl Into<String> for GetRFOutput {
    fn into(self) -> String {
        format!("$ECG,{}", self.channel)
    }
}

impl GetRFOutput {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetRFOutput {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
