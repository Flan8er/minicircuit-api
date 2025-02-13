use serde::{Deserialize, Serialize};

use crate::data_types::{errors::MWError, types::Channel};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetTimedRFEnableResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetTimedRFEnableResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetTimedRFEnableResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Initiates a single timed enable of specified duration.
pub struct SetTimedRFEnable {
    /// Channel identification number.
    pub channel: Channel,
    /// Duration of the timed enable in microseconds.
    pub duration: u32,
}

impl Into<String> for SetTimedRFEnable {
    fn into(self) -> String {
        format!("$ECST,{},1,{}", self.channel, self.duration)
    }
}

impl SetTimedRFEnable {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(channel: Channel, duration: u32) -> Self {
        Self { channel, duration }
    }
}

impl Default for SetTimedRFEnable {
    /// Returns the default handler to call the command.
    /// By default, duration is set to 5,000,000 microseconds (5 seconds).
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            duration: 5_000_000,
        }
    }
}
