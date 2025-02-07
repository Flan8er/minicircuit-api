use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Watt},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOAForwardPowerLimitsResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOAForwardPowerLimitsResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOAForwardPowerLimitsResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the forward power values at which SOA takes action in Watts.
///
/// One of the features of the SOA is protection against excessive forward power.
///
/// The SOA has two reactions to excess forward power, depending on the severity:
///
/// - If the forward power is high, but still tolerable: raise a `HighForwardPower` error.
///
/// - If the forward power is dangerously high: raise a `ShutdownForwardPower` error and shutdown RF power.
pub struct SetSOAForwardPowerLimits {
    /// Channel identification number.
    pub channel: Channel,
    /// The forward power value in dBm at which the `HighForwardPower` reaction is performed by the SOA.
    pub high_forward_power: Watt,
    /// The forward power value in dBm at which the `ShutdownForwardPower` reaction is performed by the SOA.
    pub shutdown_forward_power: Watt,
}

impl Into<String> for SetSOAForwardPowerLimits {
    fn into(self) -> String {
        format!(
            "$SFS,{},{},{}",
            self.channel, self.high_forward_power, self.shutdown_forward_power
        )
    }
}

impl SetSOAForwardPowerLimits {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(channel: Channel, high_forward_power: Watt, shutdown_forward_power: Watt) -> Self {
        Self {
            channel,
            high_forward_power,
            shutdown_forward_power,
        }
    }
}

impl Default for SetSOAForwardPowerLimits {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOAForwardPowerLimitsResponse {
    /// The forward power value in dBm at which the `HighForwardPower` reaction is performed by the SOA.
    pub high_forward_power: Watt,
    /// The forward power value in dBm at which the `ShutdownForwardPower` reaction is performed by the SOA.
    pub shutdown_forward_power: Watt,
}

impl TryFrom<String> for GetSOAForwardPowerLimitsResponse {
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
        if parts.len() != 4 {
            return Err(Self::Error::FailedParseResponse);
        }

        let high_forward_power: Watt = match parts[2].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let shutdown_forward_power: Watt = match parts[3].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetSOAForwardPowerLimitsResponse {
            high_forward_power,
            shutdown_forward_power,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the forward power values at which SOA takes action in Watts.
///
/// One of the features of the SOA is protection against excessive forward power.
///
/// The SOA has two reactions to excess forward power, depending on the severity:
///
/// - If the forward power is high, but still tolerable: raise a `HighForwardPower` error.
///
/// - If the forward power is dangerously high: raise a `ShutdownForwardPower` error and shutdown RF power.
pub struct GetSOAForwardPowerLimits {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetSOAForwardPowerLimits {
    fn into(self) -> String {
        format!("$SFG,{}", self.channel)
    }
}

impl GetSOAForwardPowerLimits {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOAForwardPowerLimits {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
