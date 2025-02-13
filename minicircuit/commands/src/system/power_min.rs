use serde::{Deserialize, Serialize};

use crate::data_types::{
    errors::MWError,
    types::{Channel, Dbm},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPowerMinDbmResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPowerMinDbmResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPowerMinDbmResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Configures a minimum output power cap. This limits the forward power setpoint
/// (`SetPAPowerSetpointWatt` / `SetPAPowerSetpointDBM`) to be no lower than the configured minimum value.
/// This minimum power limit ensures that power setting inputs stay within the valid calibration range of the instruments.
/// This is especially important when operating in feed-forward mode where the internal
/// attenuation settings are only well-defined for powers within the operating range.
pub struct SetPowerMinDbm {
    /// Channel identification number.
    pub channel: Channel,
    /// The minimum permitted forward power setting in dBm.
    pub min: Dbm,
}

impl Into<String> for SetPowerMinDbm {
    fn into(self) -> String {
        format!("$PWRMINDS,{},{}", self.channel, self.min)
    }
}

impl SetPowerMinDbm {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(channel: Channel, min: Dbm) -> Self {
        Self { channel, min }
    }
}

impl Default for SetPowerMinDbm {
    /// Returns the default handler to call the command.
    ///
    /// By default, minimum is set to -30dBm.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            min: Dbm::new(-30.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPowerMinDbmResponse {
    /// The minimum permitted forward power setting in dBm.
    pub min: Dbm,
}

impl TryFrom<String> for GetPowerMinDbmResponse {
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

        let min: Dbm = match parts[2].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPowerMinDbmResponse { min })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the minimum permitted forward power setting in dBm.
pub struct GetPowerMinDbm {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPowerMinDbm {
    fn into(self) -> String {
        format!("$PWRMINDG,{}", self.channel)
    }
}

impl GetPowerMinDbm {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPowerMinDbm {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
