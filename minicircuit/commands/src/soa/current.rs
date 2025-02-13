use serde::{Deserialize, Serialize};

use crate::data_types::{
    errors::MWError,
    types::{Amperes, Channel},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOACurrentConfigResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOACurrentConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOACurrentConfigResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the currents at which SOA takes action.
///
/// One of the features of the SOA is protection against improper
/// application of DC current. Current SOA protects against overcurrent conditions.
///
/// The SOA has two reactions to excessive current, depending on the severity:
///
/// - If the current is higher than normal operating range, but still tolerable: raise a `SOAHighCurrent` error.
///
/// - If the current is dangerously high: raise a `SOAShutdownMaximumCurrent` error and shutdown RF power.
pub struct SetSOACurrentConfig {
    /// Channel identification number.
    pub channel: Channel,
    /// The current at which the ‘SOAHighCurrent’ condition is signaled by the SOA. Units in Amps.
    pub high_current: Amperes,
    /// The current at which the ‘SOAShutdownCurrent’ condition is signaled by the SOA. Units in Amps.
    pub shutdown_current: Amperes,
}

impl Into<String> for SetSOACurrentConfig {
    fn into(self) -> String {
        format!(
            "$SCS,{},{},{}",
            self.channel, self.high_current, self.shutdown_current
        )
    }
}

impl SetSOACurrentConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(channel: Channel, high_current: Amperes, shutdown_current: Amperes) -> Self {
        Self {
            channel,
            high_current,
            shutdown_current,
        }
    }
}

impl Default for SetSOACurrentConfig {
    /// Returns the default handler to call the command.
    /// By default, high current is set to 5.5A,
    /// and shutdown temperature is set to 6A.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_current: Amperes::new(5.5),
            shutdown_current: Amperes::new(6.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOACurrentConfigResponse {
    /// The current at which the `SOAHighCurrent` condition is signaled by the SOA in Amps.
    pub high_current: Amperes,
    /// The current at which the `SOAShutdownCurrent` condition is signaled by the SOA in Amps.
    pub shutdown_current: Amperes,
}

impl TryFrom<String> for GetSOACurrentConfigResponse {
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

        let high_current: Amperes = match parts[2].trim().parse::<f32>() {
            Ok(value) => Amperes::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let shutdown_current: Amperes = match parts[3].trim().parse::<f32>() {
            Ok(value) => Amperes::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetSOACurrentConfigResponse {
            high_current,
            shutdown_current,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the currents at which SOA takes action.
///
/// One of the features of the SOA is protection against improper
/// application of DC current. Current SOA protects against overcurrent conditions.
///
/// The SOA has two reactions to excessive current, depending on the severity:
///
/// - If the current is higher than normal operating range, but still tolerable: raise a `SOAHighCurrent` error.
///
/// - If the current is dangerously high: raise a `SOAShutdownMaximumCurrent` error and shutdown RF power.
pub struct GetSOACurrentConfig {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetSOACurrentConfig {
    fn into(self) -> String {
        format!("$SCG,{}", self.channel)
    }
}

impl GetSOACurrentConfig {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOACurrentConfig {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
