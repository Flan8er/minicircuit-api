use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Temperature},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOATempConfigResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOATempConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOATempConfigResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Configures the temperature values at which SOA takes action.
/// One of the features of the SOA is protection against excessive temperatures.
/// Excessive temperatures can occur for any number of reasons: side effects of high
/// RF power reflection, faulty cooling, excessive use, etc.
///
/// The SOA has two reactions to excessive temperatures, depending on the severity:
///
/// - If the temperature is high, but still tolerable: raise a `HighTemperature` error.
///
/// - If the temperature is dangerously high: raise a `ShutdownTemperature` error and shutdown RF power.
pub struct SetSOATempConfig {
    /// Channel identification number.
    pub channel: Channel,
    /// The temperature value in deg C at which `HighTemperature` situation is signaled by the SOA.
    /// The corresponding bit in the status word is set and can be read with a `GetStatus` command.
    pub high_temp: Temperature,
    /// The temperature value in deg C at which `ShutdownTemperature` reaction is performed by the SOA.
    /// The generator will be switched off and the corresponding error but will be set in the status word.
    pub shutdown_temp: Temperature,
}

impl Into<String> for SetSOATempConfig {
    fn into(self) -> String {
        format!(
            "$STS,{},{},{}",
            self.channel, self.high_temp, self.shutdown_temp
        )
    }
}

impl SetSOATempConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(self, channel: Channel, high_temp: Temperature, shutdown_temp: Temperature) -> Self {
        Self {
            channel,
            high_temp,
            shutdown_temp,
        }
    }
}

impl Default for SetSOATempConfig {
    /// Returns the default handler to call the command.
    /// By default, high temperature is set to 55 deg C,
    /// and shutdown temperature is set to 65 deg C.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_temp: Temperature::new(55),
            shutdown_temp: Temperature::new(65),
        }
    }
}

pub struct GetSOATempConfigResponse {
    /// The temperature value in deg C at which `HighTemperature` situation is signaled by the SOA.
    /// The corresponding bit in the status word is set and can be read with a `GetStatus` command.
    pub high_temp: Temperature,
    /// The temperature value in deg C at which `ShutdownTemperature` reaction is performed by the SOA.
    /// The generator will be switched off and the corresponding error but will be set in the status word.
    pub shutdown_temp: Temperature,
}

impl TryFrom<String> for GetSOATempConfigResponse {
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

        let high_temp: Temperature = match parts[2].trim().parse::<u8>() {
            Ok(value) => Temperature::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let shutdown_temp: Temperature = match parts[3].trim().parse::<u8>() {
            Ok(value) => Temperature::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetSOATempConfigResponse {
            high_temp,
            shutdown_temp,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the temperature values at which the SOA takes action.
pub struct GetSOATempConfig {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetSOATempConfig {
    fn into(self) -> String {
        format!("$STG,{}", self.channel)
    }
}

impl GetSOATempConfig {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOATempConfig {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
