use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetChannelIDResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetChannelIDResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetChannelIDResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Assigns a channel identification number to the specified ISC board.
///
/// Every ISC board is assigned a numeric value as a challen identifier for communication.
/// The default value of the identifier is `1`, which serves its purpose in single-channel systems.
/// In setups that deploy more than one ISC board is often necessary to assign a unique number to each individual board beforehand,
/// so that they can all be commanded as seperate entities. An ISC board will not respond to commands written for a different channel.
pub struct SetChannelID {
    /// Channel identification number to change.
    pub channel: Channel,
    /// New desured channel identification number.
    pub new_channel: Channel,
}

impl Into<String> for SetChannelID {
    fn into(self) -> String {
        format!("$CHANS,{},{}", self.channel, self.new_channel)
    }
}

impl SetChannelID {
    /// Returns a handler to call the command.
    pub fn new(self, channel: Channel, new_channel: Channel) -> Self {
        Self {
            channel,
            new_channel,
        }
    }

    /// Sets the system back to it's default state.
    pub fn return_to_default(self, current_channel: Channel) -> Self {
        Self {
            channel: current_channel,
            new_channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChannelIDResponse {
    /// Channel identification number.
    pub channel: Channel,
}

impl TryFrom<String> for GetChannelIDResponse {
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
        if parts.len() != 2 {
            return Err(Self::Error::FailedParseResponse);
        }

        let channel: Channel = match parts[1].trim().parse::<u8>() {
            Ok(value) => Channel::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetChannelIDResponse { channel })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
