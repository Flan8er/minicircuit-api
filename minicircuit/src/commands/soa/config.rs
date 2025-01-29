use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SOAType {
    /// See `SetSOATempConfig`
    Temperature,
    /// See `SetSOAPowerConfig`
    Reflection,
    ExternalWatchdog,
    /// See `SetSOADissipationConfig`
    Dissipation,
    /// See `$PSG`
    PAStatus,
    IQModulator,
    /// See `SetSOACurrentConfig`
    Current,
    /// See `SetSOAVoltageConfig`
    Voltage,
    /// See `SetSOAForwardPowerLimits`
    ForwardPower,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOAConfigResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOAConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOAConfigResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Configures the enable state of the SOA's protection systems.
///
/// SOA has the following protection systems in place:
///
/// - Protection against high temperatures.
///
/// - Protections against software timeouts / freezes.
///
/// - Protection against excessive reflection.
///
/// - Auto-disable RF power if the board status is not polled frequently enough.
pub struct SetSOAConfig {
    /// Channel identification number.
    pub channel: Channel,
    pub soa_type: SOAType,
    pub enabled: bool,
}

impl Into<String> for SetSOAConfig {
    fn into(self) -> String {
        let soa_type: u8 = match self.soa_type {
            SOAType::Temperature => 0,
            SOAType::Reflection => 2,
            SOAType::ExternalWatchdog => 3,
            SOAType::Dissipation => 4,
            SOAType::PAStatus => 5,
            SOAType::IQModulator => 6,
            SOAType::Current => 7,
            SOAType::Voltage => 8,
            SOAType::ForwardPower => 9,
        };

        let enabled: u8 = match self.enabled {
            true => 1,
            false => 0,
        };

        format!("$SOA,{},{},{}", self.channel, soa_type, enabled)
    }
}

impl SetSOAConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(self, channel: Channel, soa_type: SOAType, enabled: bool) -> Self {
        Self {
            channel,
            soa_type,
            enabled,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Voltage and forward power SOA enable statuses are not shown here. View their dedicated commands:
///
/// `GetSOAVoltageLimits` and `GetSOAForwardPowerLimits`
pub struct GetSOAConfigResponse {
    pub temp_enabled: bool,
    pub reflection_enabled: bool,
    pub external_watchdog_enabled: bool,
    pub dissipation_enabled: bool,
    pub pa_status_enabled: bool,
    pub iq_modulator_enabled: bool,
    pub current_enabled: bool,
}

impl TryFrom<String> for GetSOAConfigResponse {
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
        if parts.len() != 10 {
            return Err(Self::Error::FailedParseResponse);
        }

        let temp_enabled: bool = match parts[2].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflection_enabled: bool = match parts[4].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let external_watchdog_enabled: bool = match parts[5].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let dissipation_enabled: bool = match parts[6].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let pa_status_enabled: bool = match parts[7].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let iq_modulator_enabled: bool = match parts[8].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let current_enabled: bool = match parts[9].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetSOAConfigResponse {
            temp_enabled,
            reflection_enabled,
            external_watchdog_enabled,
            dissipation_enabled,
            pa_status_enabled,
            iq_modulator_enabled,
            current_enabled,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the enable state of the SOA's protection systems.
pub struct GetSOAConfig {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetSOAConfig {
    fn into(self) -> String {
        format!("$SOG,{}", self.channel)
    }
}

impl GetSOAConfig {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOAConfig {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
