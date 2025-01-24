use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetFrequencyResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetFrequencyResponse {
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

        Ok(SetFrequencyResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetFrequency {
    channel: u8,
    frequency: u16,
}

impl Into<String> for SetFrequency {
    fn into(self) -> String {
        format!("$FCS,{},{}", self.channel, self.frequency)
    }
}

impl SetFrequency {
    pub fn new(self, channel: u8, frequency: u16) -> Self {
        Self { channel, frequency }
    }
}

impl Default for SetFrequency {
    fn default() -> Self {
        Self {
            channel: 1,
            frequency: 2450,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetFrequencyResponse {
    /// The uptime in seconds.
    pub frequency: u16,
}

impl TryFrom<String> for GetFrequencyResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let frequency = match parts[0].parse() {
            Ok(frequency) => frequency,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetFrequencyResponse { frequency })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetFrequency {
    channel: u8,
}

impl Into<String> for GetFrequency {
    fn into(self) -> String {
        format!("$FCG,{}", self.channel)
    }
}

impl GetFrequency {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetFrequency {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
