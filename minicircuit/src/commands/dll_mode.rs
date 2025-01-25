use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Frequency},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetDLLModeResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetDLLModeResponse {
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

        Ok(SetDLLModeResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetDLLMode {
    channel: Channel,
    enabled: bool,
}

impl Into<String> for SetDLLMode {
    fn into(self) -> String {
        format!("$DLES,{},{}", Into::<u8>::into(self.channel), self.enabled)
    }
}

impl SetDLLMode {
    pub fn new(self, channel: Channel, enabled: bool) -> Self {
        Self { channel, enabled }
    }
}

impl Default for SetDLLMode {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetDLLModeResponse {
    /// The uptime in seconds.
    pub enabled: bool,
}

impl TryFrom<String> for GetDLLModeResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(GetDLLModeResponse { enabled: todo!() })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetDLLMode {
    channel: Channel,
}

impl Into<String> for GetDLLMode {
    fn into(self) -> String {
        format!("$DLEG,{}", Into::<u8>::into(self.channel))
    }
}

impl GetDLLMode {
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetDLLMode {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetDLLConfigResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetDLLConfigResponse {
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

        Ok(SetDLLConfigResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetDLLConfig {
    channel: Channel,
    lower_frequency: Frequency,
    upper_frequency: Frequency,
    start_frequency: Frequency,
    step_frequency: Frequency,
    threshold: f32,
    main_delay: u16,
}

impl Into<String> for SetDLLConfig {
    fn into(self) -> String {
        format!(
            "$DLCS,{},{},{},{},{},{},{}",
            Into::<u8>::into(self.channel),
            Into::<u16>::into(self.lower_frequency),
            Into::<u16>::into(self.upper_frequency),
            Into::<u16>::into(self.start_frequency),
            Into::<u16>::into(self.step_frequency),
            self.threshold,
            self.main_delay
        )
    }
}

impl SetDLLConfig {
    pub fn new(
        self,
        channel: Channel,
        lower_frequency: Frequency,
        upper_frequency: Frequency,
        start_frequency: Frequency,
        step_frequency: Frequency,
        threshold: f32,
        main_delay: u16,
    ) -> Self {
        Self {
            channel,
            lower_frequency,
            upper_frequency,
            start_frequency,
            step_frequency,
            threshold,
            main_delay,
        }
    }
}

impl Default for SetDLLConfig {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            lower_frequency: Frequency::new(2400),
            upper_frequency: Frequency::new(2500),
            start_frequency: Frequency::new(2410),
            step_frequency: Frequency::new(5),
            threshold: 0.5,
            main_delay: 25,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetDLLConfigResponse {
    lower_frequency: Frequency,
    upper_frequency: Frequency,
    start_frequency: Frequency,
    step_frequency: Frequency,
    threshold: f32,
    main_delay: u16,
}

impl TryFrom<String> for GetDLLConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        Ok(GetDLLConfigResponse {
            lower_frequency: todo!(),
            upper_frequency: todo!(),
            start_frequency: todo!(),
            step_frequency: todo!(),
            threshold: todo!(),
            main_delay: todo!(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetDLLConfig {
    channel: Channel,
}

impl Into<String> for GetDLLConfig {
    fn into(self) -> String {
        format!("$DLC,{}", Into::<u8>::into(self.channel),)
    }
}

impl GetDLLConfig {
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetDLLConfig {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
