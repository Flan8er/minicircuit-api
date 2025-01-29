use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Watt},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOAGraceTimerResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOAGraceTimerResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOAGraceTimerResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetSOAGraceTimer {
    /// Channel identification number.
    pub channel: Channel,
    /// The forward power value in dBm at which the `HighForwardPower` reaction is performed by the SOA.
    pub high_forward_power: Watt,
    /// The forward power value in dBm at which the `ShutdownForwardPower` reaction is performed by the SOA.
    pub shutdown_forward_power: Watt,
}

impl Into<String> for SetSOAGraceTimer {
    fn into(self) -> String {
        format!(
            "$SOAGS,{},{},{}",
            self.channel, self.high_forward_power, self.shutdown_forward_power
        )
    }
}

impl SetSOAGraceTimer {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(
        self,
        channel: Channel,
        high_forward_power: Watt,
        shutdown_forward_power: Watt,
    ) -> Self {
        Self {
            channel,
            high_forward_power,
            shutdown_forward_power,
        }
    }
}

impl Default for SetSOAGraceTimer {
    /// Returns the default handler to call the command.
    /// By default, protection values are configured to 55W (47.4 dBm)
    /// and 65W (48.15 dBm) respectively.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_forward_power: Watt::new(55.),
            shutdown_forward_power: Watt::new(65.),
        }
    }
}
