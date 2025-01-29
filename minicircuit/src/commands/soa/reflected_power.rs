use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Dbm},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOAPowerConfigResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOAPowerConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOAPowerConfigResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Configures the reflected power values at which SOA takes action.
/// One of the features of SOA is protection against excessive reflected power.
/// Excessive reflection occurs when there is a bad match at the output and RF returns to the generator.
///
/// The SOA has two reactions to excessive dissipation, depending on the severity:
///
/// - If the reflection is high, but still tolerable: raise a 'HighReflection' error.
///
/// - If the reflection is dangerously high: raise a 'ShutdownReflection' error and shutdown RF power.
pub struct SetSOAPowerConfig {
    /// Channel identification number.
    pub channel: Channel,
    /// The reflection value in dBm at which the `HighReflection` situation is signaled by the SOA.
    /// It will be reported upon a GetStatus command.
    pub high_reflection: Dbm,
    /// The reflection value in dBm at which the `ShutdownReflection` reaction is performed by the SOA.
    /// RF will be switched off and the corresponding error bit will be set.
    pub shutdown_reflection: Dbm,
}

impl Into<String> for SetSOAPowerConfig {
    fn into(self) -> String {
        format!(
            "$SPS,{},{},{}",
            self.channel, self.high_reflection, self.shutdown_reflection
        )
    }
}

impl SetSOAPowerConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(self, channel: Channel, high_reflection: Dbm, shutdown_reflection: Dbm) -> Self {
        Self {
            channel,
            high_reflection,
            shutdown_reflection,
        }
    }
}

impl Default for SetSOAPowerConfig {
    /// Returns the default handler to call the command.
    /// By default, 'HighReflection' will be triggered at 47.25 dBm (53W),
    /// and 'ShutdownReflection' will be triggered at 54 dBm (55W).
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_reflection: Dbm::new(47.25),
            shutdown_reflection: Dbm::new(54.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOAPowerConfigResponse {
    /// The reflection value in dBm at which the `HighReflection` situation is signaled by the SOA.
    /// It will be reported upon a GetStatus command.
    pub high_reflection: Dbm,
    /// The reflection value in dBm at which the `ShutdownReflection` reaction is performed by the SOA.
    /// RF will be switched off and the corresponding error bit will be set.
    pub shutdown_reflection: Dbm,
}

impl TryFrom<String> for GetSOAPowerConfigResponse {
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

        let high_reflection: Dbm = match parts[2].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let shutdown_reflection: Dbm = match parts[3].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetSOAPowerConfigResponse {
            high_reflection,
            shutdown_reflection,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the reflection values at which SOA takes action.
pub struct GetSOAPowerConfig {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetSOAPowerConfig {
    fn into(self) -> String {
        format!("$SPG,{}", self.channel)
    }
}

impl GetSOAPowerConfig {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOAPowerConfig {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
