use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, DutyCycle, Frequency},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPWMDutyCycleResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetPWMDutyCycleResponse {
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

        Ok(SetPWMDutyCycleResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPWMDutyCycle {
    channel: Channel,
    duty_cycle: DutyCycle,
}

impl Into<String> for SetPWMDutyCycle {
    fn into(self) -> String {
        format!(
            "$DCS,{},{}",
            Into::<u8>::into(self.channel),
            Into::<u8>::into(self.duty_cycle)
        )
    }
}

impl SetPWMDutyCycle {
    pub fn new(self, channel: Channel, duty_cycle: DutyCycle) -> Self {
        Self {
            channel,
            duty_cycle,
        }
    }
}

impl Default for SetPWMDutyCycle {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            duty_cycle: DutyCycle::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPWMSettingsResponse {
    /// The uptime in seconds.
    pub frequency: Frequency,
    pub correction_factor: u8,
    pub duty_cycle: DutyCycle,
}

impl TryFrom<String> for GetPWMSettingsResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(GetPWMSettingsResponse {
            frequency: todo!(),
            correction_factor: todo!(),
            duty_cycle: todo!(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPWMSettings {
    channel: Channel,
}

impl Into<String> for GetPWMSettings {
    fn into(self) -> String {
        format!("$DCG,{}", Into::<u8>::into(self.channel),)
    }
}

impl GetPWMSettings {
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPWMSettings {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
