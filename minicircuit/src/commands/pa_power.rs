use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerWattResponse {
    /// The uptime in seconds.
    pub forward: f32,
    pub reflected: f32,
}

impl TryFrom<String> for GetPAPowerWattResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        // let uptime = match parts[0].parse() {
        //     Ok(uptime) => uptime,
        //     Err(_) => return Err(MWError::FailedParseResponse),
        // };

        Ok(GetPAPowerWattResponse {
            forward: 3.,
            reflected: 3.,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerWatt {
    channel: u8,
}

impl Into<String> for GetPAPowerWatt {
    fn into(self) -> String {
        format!("$PPG,{}", self.channel)
    }
}

impl GetPAPowerWatt {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerWatt {
    fn default() -> Self {
        Self { channel: 1 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerDBMResponse {
    /// The uptime in seconds.
    pub forward: f32,
    pub reflected: f32,
}

impl TryFrom<String> for GetPAPowerDBMResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        // let uptime = match parts[0].parse() {
        //     Ok(uptime) => uptime,
        //     Err(_) => return Err(MWError::FailedParseResponse),
        // };

        Ok(GetPAPowerDBMResponse {
            forward: 3.,
            reflected: 3.,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerDBM {
    channel: u8,
}

impl Into<String> for GetPAPowerDBM {
    fn into(self) -> String {
        format!("$PPDG,{}", self.channel)
    }
}

impl GetPAPowerDBM {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerDBM {
    fn default() -> Self {
        Self { channel: 1 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPAPowerSetpointWattResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetPAPowerSetpointWattResponse {
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

        Ok(SetPAPowerSetpointWattResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPAPowerSetpointWatt {
    channel: u8,
    power: u16,
}

impl Into<String> for SetPAPowerSetpointWatt {
    fn into(self) -> String {
        format!("$PWRS,{},{}", self.channel, self.power)
    }
}

impl SetPAPowerSetpointWatt {
    pub fn new(self, channel: u8, power: u16) -> Self {
        Self { channel, power }
    }
}

impl Default for SetPAPowerSetpointWatt {
    fn default() -> Self {
        Self {
            channel: 1,
            power: 250,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerSetpointWattResponse {
    /// The uptime in seconds.
    pub power: u16,
}

impl TryFrom<String> for GetPAPowerSetpointWattResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let power = match parts[0].parse() {
            Ok(power) => power,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetPAPowerSetpointWattResponse { power })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerSetpointWatt {
    channel: u8,
}

impl Into<String> for GetPAPowerSetpointWatt {
    fn into(self) -> String {
        format!("$PWRG,{}", self.channel)
    }
}

impl GetPAPowerSetpointWatt {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerSetpointWatt {
    fn default() -> Self {
        Self { channel: 1 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPAPowerSetpointDBMResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetPAPowerSetpointDBMResponse {
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

        Ok(SetPAPowerSetpointDBMResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPAPowerSetpointDBM {
    channel: u8,
    power: u8,
}

impl Into<String> for SetPAPowerSetpointDBM {
    fn into(self) -> String {
        format!("$PWRDS,{},{}", self.channel, self.power)
    }
}

impl SetPAPowerSetpointDBM {
    pub fn new(self, channel: u8, power: u8) -> Self {
        Self { channel, power }
    }
}

impl Default for SetPAPowerSetpointDBM {
    fn default() -> Self {
        Self {
            channel: 1,
            power: 50,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerSetpointDBMResponse {
    /// The uptime in seconds.
    pub power: u16,
}

impl TryFrom<String> for GetPAPowerSetpointDBMResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let power = match parts[0].parse() {
            Ok(power) => power,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetPAPowerSetpointDBMResponse { power })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerSetpointDBM {
    channel: u8,
}

impl Into<String> for GetPAPowerSetpointDBM {
    fn into(self) -> String {
        format!("$PWRDG,{}", self.channel)
    }
}

impl GetPAPowerSetpointDBM {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerSetpointDBM {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
