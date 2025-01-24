use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IdentityResponse {
    /// Name of the manufacturer.
    pub manufacturer: String,
    /// The type of ISC board.
    pub isc_board: String,
    /// Unique serial number of the board.
    pub serial_number: String,
}

impl TryFrom<String> for IdentityResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(IdentityResponse {
            manufacturer: parts[0].to_string(),
            isc_board: parts[1].to_string(),
            serial_number: parts[2].to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetIdentity {
    channel: u8,
}

impl Into<String> for GetIdentity {
    fn into(self) -> String {
        format!("$IDN,{}", self.channel)
    }
}

impl GetIdentity {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetIdentity {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
