use serde::{Deserialize, Serialize};

use crate::data_types::{
    errors::MWError,
    types::{Channel, Dbm, Watt},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPAPowerSetpointWattResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPAPowerSetpointWattResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPAPowerSetpointWattResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the amplifier chain's output power setpoint to the desired value in watts.
pub struct SetPAPowerSetpointWatt {
    /// Channel identification number.
    pub channel: Channel,
    /// Desired power setpoint.
    pub power: Watt,
}

impl Into<String> for SetPAPowerSetpointWatt {
    fn into(self) -> String {
        format!("$PWRS,{},{}", self.channel, self.power)
    }
}

impl SetPAPowerSetpointWatt {
    /// Returns a handler to call the command.
    pub fn new(channel: Channel, power: Watt) -> Self {
        Self { channel, power }
    }
}

impl Default for SetPAPowerSetpointWatt {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            power: Watt::new(250.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerSetpointWattResponse {
    /// The current output power value for the RF signal in watt.
    pub power: Watt,
}

impl TryFrom<String> for GetPAPowerSetpointWattResponse {
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

        let power: Watt = match parts[2].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAPowerSetpointWattResponse { power })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the configured output power setpoint in watts.
pub struct GetPAPowerSetpointWatt {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAPowerSetpointWatt {
    fn into(self) -> String {
        format!("$PWRG,{}", self.channel)
    }
}

impl GetPAPowerSetpointWatt {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerSetpointWatt {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPAPowerSetpointDBMResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPAPowerSetpointDBMResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPAPowerSetpointDBMResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the output power setpoint to the desired value in dBm.
pub struct SetPAPowerSetpointDBM {
    /// Channel identification number.
    pub channel: Channel,
    /// Desired power value for the RF signal in dBm.
    pub power: Dbm,
}

impl Into<String> for SetPAPowerSetpointDBM {
    fn into(self) -> String {
        format!("$PWRDS,{},{}", self.channel, self.power)
    }
}

impl SetPAPowerSetpointDBM {
    /// Returns a handler to call the command.
    pub fn new(channel: Channel, power: Dbm) -> Self {
        Self { channel, power }
    }
}

impl Default for SetPAPowerSetpointDBM {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            power: Dbm::new(50.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerSetpointDBMResponse {
    /// The current power value for the RF signal in dBm.
    pub power: Dbm,
}

impl TryFrom<String> for GetPAPowerSetpointDBMResponse {
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

        Ok(GetPAPowerSetpointDBMResponse { power })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the configured output power setpoint in dBm.
pub struct GetPAPowerSetpointDBM {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAPowerSetpointDBM {
    fn into(self) -> String {
        format!("$PWRDG,{}", self.channel)
    }
}

impl GetPAPowerSetpointDBM {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerSetpointDBM {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
