use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPowerOffsetResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPowerOffsetResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPowerOffsetResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Sets the power offset of the system.
///
/// Power offset is used when there is a fixed attenuation at the output
/// of the generator and the user would like to see power referenced to the
/// plane after that attenuation. For example, an offset setting of 3 would mean
/// that there is 3bB of loss between the generator output and the new reference plane.
///
/// This affects the behavior of several functions:
///
/// - `GetPAPowerWatt` and `GetPAPowerDBM` normally return the forward and reflected powers.
///  Now forward powers are reduces by the offset value (in dB) and the reflected powers are
/// increased by the offset value (in dB). Note that this means that any calculaton of Return
/// loss will be 2 * offset (dB) lower than normal.
///
/// - In both auto-gain and feed-forward modes, `SetPAPowerSetpointWatt` and `SetPAPowerSetpointDBM`
/// are now referencing the power at the new reference plane. The minimum and maximum power settings
/// are adjusted accordingly (reduced by the offset).
pub struct SetPowerOffset {
    /// Channel identification number.
    pub channel: Channel,
    /// Desired power offset in dB.
    pub offset: u8,
}

impl Into<String> for SetPowerOffset {
    fn into(self) -> String {
        format!("$PODS,{},{}", self.channel, self.offset)
    }
}

impl SetPowerOffset {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(self, channel: Channel, offset: u8) -> Self {
        Self { channel, offset }
    }
}

impl Default for SetPowerOffset {
    /// Returns the default handler to call the command.
    ///
    /// By default, offset is set to 0dB.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            offset: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetPowerOffsetResponse {
    /// The offset value of the system in dB.
    pub offset: u8,
}

impl TryFrom<String> for GetPowerOffsetResponse {
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

        let offset = match parts[2].trim().parse::<u8>() {
            Ok(value) => value,
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPowerOffsetResponse { offset })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the power offset of the system in dB.
pub struct GetPowerOffset {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPowerOffset {
    fn into(self) -> String {
        format!("$PODG,{}", self.channel)
    }
}

impl GetPowerOffset {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPowerOffset {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
