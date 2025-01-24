use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPATempResponse {
    /// The uptime in seconds.
    pub temperature: u16,
}

impl TryFrom<String> for GetPATempResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let temperature = match parts[0].parse() {
            Ok(temperature) => temperature,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetPATempResponse { temperature })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// This command returns the temperature of the power amplifier (PA).
pub struct GetPATemp {
    channel: u8,
}

impl Into<String> for GetPATemp {
    fn into(self) -> String {
        format!("$PTG,{}", self.channel)
    }
}

impl GetPATemp {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetPATemp {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
