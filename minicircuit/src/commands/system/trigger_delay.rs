use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetZHLTriggerDelayResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetZHLTriggerDelayResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetZHLTriggerDelayResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Sets the trigger delay on the ZHL in units of μs. Refer to the device data sheet
/// for details on this parameter. The ISC board sends triggers to trigger measurements
/// while PWM, DLL, or Sweep features are active. This delay parameter should generally not
/// be changed.
pub struct SetZHLTriggerDelay {
    /// Channel identification number.
    pub channel: Channel,
    /// Trigger delay on the ZHL in units of μs.
    /// This is the delay between receiving a trigger and performing an ADC acquisition.
    pub delay: u16,
}

impl Into<String> for SetZHLTriggerDelay {
    fn into(self) -> String {
        format!("$ZHLDS,{},{}", self.channel, self.delay)
    }
}

impl SetZHLTriggerDelay {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(channel: Channel, delay: u16) -> Self {
        Self { channel, delay }
    }
}

impl Default for SetZHLTriggerDelay {
    /// Returns the default handler to call the command.
    ///
    /// By default, the delay will be set to 30μs.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            delay: 30,
        }
    }
}
