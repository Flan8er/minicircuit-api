use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Dbm},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPowerMaxDbmResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPowerMaxDbmResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPowerMaxDbmResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Configures a maximum output power cap. This prevents inputting a forward power setpoint
/// (`SetPAPowerSetpointWatt` / `SetPAPowerSetpointDBM`) beyond the configured maximum value.
/// Useful for configuring or ignoring limits in special situations.
pub struct SetPowerMaxDbm {
    /// Channel identification number.
    pub channel: Channel,
    /// The maximum permitted forward power setting in dBm.
    pub max: Dbm,
}

impl Into<String> for SetPowerMaxDbm {
    fn into(self) -> String {
        format!("$PWRMDS,{},{}", self.channel, self.max)
    }
}

impl SetPowerMaxDbm {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(self, channel: Channel, max: Dbm) -> Self {
        Self { channel, max }
    }
}

impl Default for SetPowerMaxDbm {
    /// Returns the default handler to call the command.
    ///
    /// By default, maximum is set to 47.1dBm.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            max: Dbm::new(47.1),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPowerMaxDbmResponse {
    /// The maximum permitted forward power setting in dBm.
    pub max: Dbm,
}

impl TryFrom<String> for GetPowerMaxDbmResponse {
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

        let max: Dbm = match parts[2].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPowerMaxDbmResponse { max })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the maximum permitted forward power setting in dBm.
pub struct GetPowerMaxDbm {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPowerMaxDbm {
    fn into(self) -> String {
        format!("$PWRMDG,{}", self.channel)
    }
}

impl GetPowerMaxDbm {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPowerMaxDbm {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
