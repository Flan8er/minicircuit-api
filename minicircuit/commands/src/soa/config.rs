use serde::{Deserialize, Serialize};

use crate::data_types::{errors::MWError, types::Channel};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    /// Enable state of the board status polling protection system.
    pub external_watchdog_enabled: bool,
    /// Enable state of the dissipation protection, i.e., a maximum amount of dissipated
    /// power inside the amplifier can be set. The dissipated power is the sum of the reflected
    /// RF power and the dissipation due to the RF generation process.
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Voltage and forward power SOA enable statuses are not shown here. View their dedicated commands:
///
/// `GetSOAVoltageLimits` and `GetSOAForwardPowerLimits`
pub struct GetSOAConfigResponse {
    pub temp_enabled: bool,
    pub reflection_enabled: bool,
    pub external_watchdog_enabled: bool,
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
        if parts.len() != 4 {
            return Err(Self::Error::FailedParseResponse);
        }

        let temp_parts: Vec<&str> = parts[1].split(":").collect();
        let reflection_parts: Vec<&str> = parts[2].split(":").collect();
        let watchdog_parts: Vec<&str> = parts[3].split(":").collect();
        if temp_parts.len() != 2 || reflection_parts.len() != 2 || watchdog_parts.len() != 2 {
            return Err(Self::Error::FailedParseResponse);
        }
        let temp_enabled: bool = match temp_parts[1].split('.').collect::<Vec<&str>>()[0]
            .trim()
            .parse::<u8>()
        {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflection_enabled: bool = match reflection_parts[1].split('.').collect::<Vec<&str>>()
            [0]
        .trim()
        .parse::<u8>()
        {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let external_watchdog_enabled: bool =
            match watchdog_parts[1].split('.').collect::<Vec<&str>>()[0]
                .trim()
                .parse::<u8>()
            {
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
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    pub fn new(channel: Channel) -> Self {
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
