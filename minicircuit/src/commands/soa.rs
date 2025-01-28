use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Dbm, Temperature},
    errors::MWError,
};

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
    /// Enable state of the temperature protection system.
    pub temp_enabled: bool,
    /// Enable state of the RF power reflection protection system.
    pub reflection_enabled: bool,
    /// Enable state of teh board status polling protection.
    pub external_watchdog_enabled: bool,
    /// Enables the dissipation protection, i.e., a maximum amount of dissipated
    /// power inside the amplifier can be set. The dissipated power is the sum
    /// of the reflected RF power and the dissipation due to the RF generation process.
    pub dissipation_enabled: bool,
}

impl Into<String> for SetSOAConfig {
    fn into(self) -> String {
        let temp_enabled: u8 = match self.temp_enabled {
            true => 1,
            false => 0,
        };
        let reflection_enabled: u8 = match self.reflection_enabled {
            true => 1,
            false => 0,
        };
        let external_watchdog_enabled: u8 = match self.external_watchdog_enabled {
            true => 1,
            false => 0,
        };
        let dissipation_enabled: u8 = match self.dissipation_enabled {
            true => 1,
            false => 0,
        };

        format!(
            "$SOA,{},{},1,{},{},{}",
            self.channel,
            temp_enabled,
            reflection_enabled,
            external_watchdog_enabled,
            dissipation_enabled
        )
    }
}

impl SetSOAConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(
        self,
        channel: Channel,
        temp_enabled: bool,
        reflection_enabled: bool,
        external_watchdog_enabled: bool,
        dissipation_enabled: bool,
    ) -> Self {
        Self {
            channel,
            temp_enabled,
            reflection_enabled,
            external_watchdog_enabled,
            dissipation_enabled,
        }
    }
}

impl Default for SetSOAConfig {
    /// Returns the default handler to call the command.
    /// By default, all protections are enabled.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            temp_enabled: true,
            reflection_enabled: true,
            external_watchdog_enabled: true,
            dissipation_enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOAConfigResponse {
    pub temp_enabled: bool,
    pub reflection_enabled: bool,
    pub external_watchdog_enabled: bool,
    pub dissipation_enabled: bool,
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
        if parts.len() != 5 {
            return Err(Self::Error::FailedParseResponse);
        }

        // Split the index at the ':' delimiter
        let temp_enabled: bool = match parts[1].split_once(":") {
            Some((key, value)) => {
                // Ensure the temperature_enabled section is being parsed
                match key.trim() {
                    "Tmp" => (),
                    _ => {
                        return Err(Self::Error::FailedParseResponse);
                    }
                };

                // Transform 1/0 into true/false
                match value.trim().parse::<u8>() {
                    // Set the temp_enabled field to it's respective value
                    Ok(value) => match value {
                        1 => true,
                        _ => false,
                    },
                    Err(_) => {
                        return Err(Self::Error::FailedParseResponse);
                    }
                }
            }
            None => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflection_enabled: bool = match parts[2].split_once(":") {
            Some((key, value)) => {
                // Ensure the temperature_enabled section is being parsed
                match key.trim() {
                    "S11" => (),
                    _ => {
                        return Err(Self::Error::FailedParseResponse);
                    }
                };

                // Transform 1/0 into true/false
                match value.trim().parse::<u8>() {
                    // Set the temp_enabled field to it's respective value
                    Ok(value) => match value {
                        1 => true,
                        _ => false,
                    },
                    Err(_) => {
                        return Err(Self::Error::FailedParseResponse);
                    }
                }
            }
            None => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let external_watchdog_enabled: bool = match parts[1].split_once(":") {
            Some((key, value)) => {
                // Ensure the temperature_enabled section is being parsed
                match key.trim() {
                    "eWD" => (),
                    _ => {
                        return Err(Self::Error::FailedParseResponse);
                    }
                };

                // Transform 1/0 into true/false
                match value.trim().parse::<u8>() {
                    // Set the temp_enabled field to it's respective value
                    Ok(value) => match value {
                        1 => true,
                        _ => false,
                    },
                    Err(_) => {
                        return Err(Self::Error::FailedParseResponse);
                    }
                }
            }
            None => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let dissipation_enabled: bool = match parts[1].split_once(":") {
            Some((key, value)) => {
                // Ensure the temperature_enabled section is being parsed
                match key.trim() {
                    "Diss" => (),
                    _ => {
                        return Err(Self::Error::FailedParseResponse);
                    }
                };

                // Transform 1/0 into true/false
                match value.trim().parse::<u8>() {
                    // Set the temp_enabled field to it's respective value
                    Ok(value) => match value {
                        1 => true,
                        _ => false,
                    },
                    Err(_) => {
                        return Err(Self::Error::FailedParseResponse);
                    }
                }
            }
            None => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetSOAConfigResponse {
            temp_enabled,
            reflection_enabled,
            external_watchdog_enabled,
            dissipation_enabled,
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
    /// By default, 'HighReflection' will be triggered at 53 dBm,
    /// and 'ShutdownReflection' will be triggered at 54 dBm.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_reflection: Dbm::new(53.),
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    /// By default, high temperature is set to 80 deg C,
    /// and shutdown temperature is set to 90 deg C.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_temp: Temperature::new(80),
            shutdown_temp: Temperature::new(90),
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
