use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Seconds},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// The uptime of the ISC board since its initialization. The uptime count restarts when the board is
/// reset.
pub struct GetUptimeResponse {
    /// The uptime of the board in seconds.
    pub uptime: Seconds,
}

impl TryFrom<String> for GetUptimeResponse {
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
        if parts.len() != 3 {
            return Err(Self::Error::FailedParseResponse);
        }

        let uptime = match parts[2].trim().parse::<u64>() {
            Ok(value) => value,
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetUptimeResponse {
            uptime: Seconds::new(uptime),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the uptime of the ISC board since its initialization.
/// The uptime count restarts when the board is reset.
pub struct GetUptime {
    /// Desired channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetUptime {
    fn into(self) -> String {
        format!("$RTG,{}", self.channel)
    }
}

impl GetUptime {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetUptime {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
