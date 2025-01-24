use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetChannelIDResponse {
    /// The result of the command (Ok/Err)
    pub result: String,
}

impl TryFrom<String> for SetChannelIDResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 1 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let result = match parts[0].parse() {
            Ok(result) => result,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(SetChannelIDResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetChannelID {
    channel: u8,
    new_channel: u8,
}

impl Into<String> for SetChannelID {
    fn into(self) -> String {
        format!("$CHANS,{},{}", self.channel, self.new_channel)
    }
}

impl SetChannelID {
    pub fn new(self, channel: u8, new_channel: u8) -> Self {
        Self {
            channel,
            new_channel,
        }
    }
}

impl Default for SetChannelID {
    fn default() -> Self {
        Self {
            channel: 1,
            new_channel: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetChannelIDResponse {
    /// Channel identification number.
    pub channel: u8,
}

impl TryFrom<String> for GetChannelIDResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 1 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let channel = match parts[0].parse() {
            Ok(result) => result,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetChannelIDResponse { channel })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetChannelID {}

impl Into<String> for GetChannelID {
    fn into(self) -> String {
        "$CHANG".to_string()
    }
}

impl GetChannelID {
    pub fn new(self) -> Self {
        Self {}
    }
}

impl Default for GetChannelID {
    fn default() -> Self {
        Self {}
    }
}
