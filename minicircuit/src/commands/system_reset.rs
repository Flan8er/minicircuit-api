use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResetSystemResponse {
    /// The result of the command (Ok/Err)
    pub result: String,
}

impl TryFrom<String> for ResetSystemResponse {
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

        Ok(ResetSystemResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResetSystem {
    channel: u8,
}

impl Into<String> for ResetSystem {
    fn into(self) -> String {
        format!("$RST,{}", self.channel)
    }
}

impl ResetSystem {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for ResetSystem {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
