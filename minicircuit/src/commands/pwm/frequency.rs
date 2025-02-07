use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Frequency},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPWMFrequencyResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPWMFrequencyResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPWMFrequencyResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Sets the frequency of the PWM signal.
pub struct SetPWMFrequency {
    /// Channel identification number.
    pub channel: Channel,
    /// PWM frequency in Hz.
    pub frequency: Frequency,
}

impl Into<String> for SetPWMFrequency {
    fn into(self) -> String {
        format!("$DCFS,{},{},0", self.channel, self.frequency)
    }
}

impl SetPWMFrequency {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(channel: Channel, frequency: Frequency) -> Self {
        Self { channel, frequency }
    }
}

impl Default for SetPWMFrequency {
    /// Returns the default handler to call the command.
    /// By default, frequency will be set to 1200 Hz.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            frequency: Frequency::new(1200),
        }
    }
}
