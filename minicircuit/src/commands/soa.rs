use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Dbm, DutyCycle, Frequency, Temperature},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetSOAConfigResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetSOAConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let result = match parts[0].parse() {
            Ok(result) => result,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(SetSOAConfigResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetSOAConfig {
    pub channel: Channel,
    pub temp_enabled: bool,
    reflection_enabled: bool,
    external_watchdog_enabled: bool,
    dissipation_enabled: bool,
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
            "$SOA,{},{},{},{},{}",
            Into::<u8>::into(self.channel),
            temp_enabled,
            reflection_enabled,
            external_watchdog_enabled,
            dissipation_enabled
        )
    }
}

impl SetSOAConfig {
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
    external_watchdog_enabled: bool,
    dissipation_enabled: bool,
}

impl TryFrom<String> for GetSOAConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(GetSOAConfigResponse {
            temp_enabled: todo!(),
            reflection_enabled: todo!(),
            external_watchdog_enabled: todo!(),
            dissipation_enabled: todo!(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOAConfig {
    channel: Channel,
}

impl Into<String> for GetSOAConfig {
    fn into(self) -> String {
        format!("$SOG,{}", Into::<u8>::into(self.channel),)
    }
}

impl GetSOAConfig {
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOAConfig {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetSOAPowerConfigResponse {
    pub result: String,
}

impl TryFrom<String> for SetSOAPowerConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(SetSOAPowerConfigResponse { result: todo!() })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetSOAPowerConfig {
    channel: Channel,
    high_reflection: Dbm,
    shutdown_reflection: Dbm,
}

impl Into<String> for SetSOAPowerConfig {
    fn into(self) -> String {
        format!(
            "$SPS,{},{},{}",
            Into::<u8>::into(self.channel),
            Into::<u8>::into(self.high_reflection),
            Into::<u8>::into(self.shutdown_reflection)
        )
    }
}

impl SetSOAPowerConfig {
    pub fn new(self, channel: Channel, high_reflection: Dbm, shutdown_reflection: Dbm) -> Self {
        Self {
            channel,
            high_reflection,
            shutdown_reflection,
        }
    }
}

impl Default for SetSOAPowerConfig {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_reflection: Dbm::new(53),
            shutdown_reflection: Dbm::new(54),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOAPowerConfigResponse {
    high_reflection: Dbm,
    shutdown_reflection: Dbm,
}

impl TryFrom<String> for GetSOAPowerConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(GetSOAPowerConfigResponse {
            high_reflection: todo!(),
            shutdown_reflection: todo!(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOAPowerConfig {
    channel: Channel,
}

impl Into<String> for GetSOAPowerConfig {
    fn into(self) -> String {
        format!("$SPG,{}", Into::<u8>::into(self.channel),)
    }
}

impl GetSOAPowerConfig {
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOAPowerConfig {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

pub struct SetSOATempConfigResponse {
    pub result: String,
}

impl TryFrom<String> for SetSOATempConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(SetSOATempConfigResponse { result: todo!() })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetSOATempConfig {
    channel: Channel,
    high_temp: Temperature,
    shutdown_temp: Temperature,
}

impl Into<String> for SetSOATempConfig {
    fn into(self) -> String {
        format!(
            "$STS,{},{},{}",
            Into::<u8>::into(self.channel),
            Into::<u8>::into(self.high_temp),
            Into::<u8>::into(self.shutdown_temp)
        )
    }
}

impl SetSOATempConfig {
    pub fn new(self, channel: Channel, high_temp: Temperature, shutdown_temp: Temperature) -> Self {
        Self {
            channel,
            high_temp,
            shutdown_temp,
        }
    }
}

impl Default for SetSOATempConfig {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_temp: Temperature::new(80),
            shutdown_temp: Temperature::new(90),
        }
    }
}

pub struct GetSOATempConfigResponse {
    pub high_temp: Temperature,
    pub shutdown_temp: Temperature,
}

impl TryFrom<String> for GetSOATempConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(GetSOATempConfigResponse {
            high_temp: todo!(),
            shutdown_temp: todo!(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOATempConfig {
    channel: Channel,
}

impl Into<String> for GetSOATempConfig {
    fn into(self) -> String {
        format!("$STG,{}", Into::<u8>::into(self.channel),)
    }
}

impl GetSOATempConfig {
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOATempConfig {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
