use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetUptimeResponse {
    /// The uptime in seconds.
    pub uptime: u64,
}

impl TryFrom<String> for GetUptimeResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let uptime = match parts[0].parse() {
            Ok(uptime) => uptime,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetUptimeResponse { uptime })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetUptime {
    channel: u8,
}

impl Into<String> for GetUptime {
    fn into(self) -> String {
        format!("$RTG,{}", self.channel)
    }
}

impl GetUptime {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetUptime {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
