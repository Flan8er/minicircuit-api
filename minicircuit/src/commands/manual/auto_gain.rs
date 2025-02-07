use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetAutoGainStateResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetAutoGainStateResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetAutoGainStateResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Turns the auto-gain algorithm ON or OFF.
///
/// The auto-gain algorithm automatically regulates the power output of the ISC board by configuring the DSA and Modulator bias
/// according to calibrations that are stored in the device's EEPROM and feedback from the PA.
///
/// When auto-gain is enabled, the user can simply request an arbitrary amount of power (in Watt / dBm)
/// from their RF system, and the requested power will be accurately generated (as long
/// as the calibration is good and there are no unexpected interferences).
///
/// When auto-gain is disabled, the user can take manual control of the DSA and Modulator bias.
/// Operating manually is not recommended in most situations but can be useful for troubleshooting
/// and characterizing RF systems.
///
/// Disabling auto-gain has consequences for a variety of commands:
///
/// - `SetPAPowerSetpointDBM` and `SetPAPowerSetpointWatt`set the DSA state according to the static feed-forward calibration
/// stored in the EEPROM.
///
/// - Power can be regulated manually using commands like `SetQIMagPercent` and `SetVGAAttenuationDB` to control
/// the DSA and Modulator bias directly.
///
/// - `PerformSweepWatt` and `PerformSweepDBM` ignore the "Sweet Power" argument. Sweeps are performed at whatever power output is configured
/// through the DSA and IQ modulator at the time.
pub struct SetAutoGainState {
    /// Channel identification number.
    pub channel: Channel,
    /// Desired enable state of the auto-gain algorithm.
    pub enabled: bool,
}

impl Into<String> for SetAutoGainState {
    fn into(self) -> String {
        format!("$AGES,{},{}", self.channel, self.enabled)
    }
}

impl SetAutoGainState {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(channel: Channel, enabled: bool) -> Self {
        Self { channel, enabled }
    }
}

impl Default for SetAutoGainState {
    /// Returns the default handler to call the command.
    ///
    /// By default, auto-gain is enabled.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetAutoGainStateResponse {
    /// Current enable state of the auto-gain algorithm.
    ///
    /// True = ON
    ///
    /// Fale = OFF
    pub enabled: bool,
}

impl TryFrom<String> for GetAutoGainStateResponse {
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

        let enabled: bool = match parts[2].split('.').collect::<Vec<&str>>()[0]
            .trim()
            .parse::<u8>()
        {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetAutoGainStateResponse { enabled })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the enable state of the auto-gain algorithm.
pub struct GetAutoGainState {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetAutoGainState {
    fn into(self) -> String {
        format!("$AGEG,{}", self.channel)
    }
}

impl GetAutoGainState {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetAutoGainState {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
