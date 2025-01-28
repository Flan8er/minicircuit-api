use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Dbm},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetISCPowerOutputResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetISCPowerOutputResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetISCPowerOutputResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// To use this command, auto-gain must be disabled (command not documented)
///
/// Provides a coarse method to regulate the small signal output power of the
/// ISC board by automatically configuring the values of the VGA and IQ modulator
/// to the roughly desired dBm value.
pub struct SetISCPowerOutput {
    /// Channel identification number.
    pub channel: Channel,
    /// The desired small signal output in dBm.
    pub power_dbm: Dbm,
}

impl Into<String> for SetISCPowerOutput {
    fn into(self) -> String {
        format!("$PWRSGDS,{},{}", self.channel, self.power_dbm)
    }
}

impl SetISCPowerOutput {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(self, channel: Channel, power_dbm: Dbm) -> Self {
        Self { channel, power_dbm }
    }
}

impl Default for SetISCPowerOutput {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            power_dbm: Dbm::new(20.),
        }
    }
}
