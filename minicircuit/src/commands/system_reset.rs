use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetSystemResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for ResetSystemResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(ResetSystemResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Executes a reset of the ISC board.
/// All board settings will return to their default states.
///
/// Following a reset, whether intentional or as the result of a fault,
/// the `reset detected` error flag (0x20) will be raised.
pub struct ResetSystem {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for ResetSystem {
    fn into(self) -> String {
        format!("$RST,{}", self.channel)
    }
}

impl ResetSystem {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for ResetSystem {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
