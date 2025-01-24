use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPhaseResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetPhaseResponse {
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

        Ok(SetPhaseResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPhase {
    channel: u8,
    phase: u16,
}

impl Into<String> for SetPhase {
    fn into(self) -> String {
        format!("$PCS,{},{}", self.channel, self.phase)
    }
}

impl SetPhase {
    pub fn new(self, channel: u8, phase: u16) -> Self {
        Self { channel, phase }
    }
}

impl Default for SetPhase {
    fn default() -> Self {
        Self {
            channel: 1,
            phase: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPhaseResponse {
    /// The uptime in seconds.
    pub phase: u16,
}

impl TryFrom<String> for GetPhaseResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let phase = match parts[0].parse() {
            Ok(phase) => phase,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetPhaseResponse { phase })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPhase {
    channel: u8,
}

impl Into<String> for GetPhase {
    fn into(self) -> String {
        format!("$PCG,{}", self.channel)
    }
}

impl GetPhase {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetPhase {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
