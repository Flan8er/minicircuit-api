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
/// TO USE THIS COMMAND, `SetAutoGain` MUST BE DISABLED FIRST
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
    pub fn new(channel: Channel, power_dbm: Dbm) -> Self {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetISCPowerOutputResponse {
    /// The last configured small signal output power setting in dBm.
    pub power: Dbm,
}

impl TryFrom<String> for GetISCPowerOutputResponse {
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

        let power: Dbm = match parts[2].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetISCPowerOutputResponse { power })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the last power set. The last power set does not indicate
/// the current state of the VGA and IQ Modulator which could have changed due to
/// calls to `SetMagnitude`, `SetAttenuation`, or any other function
/// that affects these settings.
pub struct GetISCPowerOutput {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetISCPowerOutput {
    fn into(self) -> String {
        format!("$PWRSGDG,{}", self.channel)
    }
}

impl GetISCPowerOutput {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetISCPowerOutput {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
