use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Volts},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOAVoltageConfigResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOAVoltageConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOAVoltageConfigResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the voltages at which the SOA takes action. One of the features of the SOA
/// is protection against improper application of DC voltage. Voltage SOA protects
/// against both undervoltage and overvoltage conditions.
///
/// The SOA has two reactions to excessive voltage, depending on the severity:
///
/// - If the voltage is outside of the normal operating range, but still tolerable: raise a `SOAHighVoltage` or `SOALowVoltage` error.
///
/// - If the voltage is dangerously low or high: raise a `SOAShutdownMinimumVoltage` or `SOAShutdownMaximumVoltage` error and shutdown RF power.
pub struct SetSOAVoltageConfig {
    /// Channel identification number.
    pub channel: Channel,
    /// The voltage at which the `MinVoltageShutdown` condition is signaled by the SOA. Units in Volts.
    pub shutdown_min_voltage: Volts,
    /// The voltage at which the `LowVoltage` condition is signaled by the SOA. Units in Volts.
    pub low_voltage: Volts,
    /// The voltage at which the `HighVoltage` condition is signaled by the SOA. Units in Volts.
    pub high_voltage: Volts,
    /// The voltage at which the `MaxVoltageShutdown` condition is signaled by the SOA. Units in Volts.
    pub shutdown_max_voltage: Volts,
}

impl Into<String> for SetSOAVoltageConfig {
    fn into(self) -> String {
        format!(
            "$SVS,{},{},{},{},{}",
            self.channel,
            self.shutdown_min_voltage,
            self.low_voltage,
            self.high_voltage,
            self.shutdown_max_voltage
        )
    }
}

impl SetSOAVoltageConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(
        channel: Channel,
        shutdown_min_voltage: Volts,
        low_voltage: Volts,
        high_voltage: Volts,
        shutdown_max_voltage: Volts,
    ) -> Self {
        Self {
            channel,
            shutdown_min_voltage,
            low_voltage,
            high_voltage,
            shutdown_max_voltage,
        }
    }
}

impl Default for SetSOAVoltageConfig {
    /// Returns the default handler to call the command.
    /// By default, limits are configured:
    ///
    /// - Shutdown min voltage: 24V
    ///
    /// - Low voltage: 26V
    ///
    /// - High voltage: 30V
    ///
    /// - Shutdown high voltage: 32V
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            shutdown_min_voltage: Volts::new(24.),
            low_voltage: Volts::new(26.),
            high_voltage: Volts::new(30.),
            shutdown_max_voltage: Volts::new(32.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Voltages at which the SOA takes action.
pub struct GetSOAVoltageConfigResponse {
    /// The voltage at which the `MinVoltageShutdown` condition is signaled by the SOA. Units in Volts.
    pub shutdown_min_voltage: Volts,
    /// The voltage at which the `LowVoltage` condition is signaled by the SOA. Units in Volts.
    pub low_voltage: Volts,
    /// The voltage at which the `HighVoltage` condition is signaled by the SOA. Units in Volts.
    pub high_voltage: Volts,
    /// The voltage at which the `MaxVoltageShutdown` condition is signaled by the SOA. Units in Volts.
    pub shutdown_max_voltage: Volts,
}

impl TryFrom<String> for GetSOAVoltageConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // First, check for errors in the response
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        // If there are no errors parse the response into struct components
        let parts: Vec<&str> = response.split_whitespace().collect();

        // Ensure the input has the expected number of parts
        if parts.len() != 6 {
            return Err(Self::Error::FailedParseResponse);
        }

        let shutdown_min_voltage: Volts = match parts[2].trim().parse::<f32>() {
            Ok(value) => Volts::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let low_voltage: Volts = match parts[3].trim().parse::<f32>() {
            Ok(value) => Volts::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let high_voltage: Volts = match parts[4].trim().parse::<f32>() {
            Ok(value) => Volts::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let shutdown_max_voltage: Volts = match parts[5].trim().parse::<f32>() {
            Ok(value) => Volts::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetSOAVoltageConfigResponse {
            shutdown_min_voltage,
            low_voltage,
            high_voltage,
            shutdown_max_voltage,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the enable state of the SOA's protection systems.
pub struct GetSOAVoltageConfig {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetSOAVoltageConfig {
    fn into(self) -> String {
        format!("$SVG,{}", self.channel)
    }
}

impl GetSOAVoltageConfig {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOAVoltageConfig {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
