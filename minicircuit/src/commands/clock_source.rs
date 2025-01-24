use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetClockSourceResponse {
    /// The result of the command (Ok/Err)
    pub result: String,
}

impl TryFrom<String> for SetClockSourceResponse {
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

        Ok(SetClockSourceResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetClockSource {
    channel: u8,
    clock_source: ClockSource,
}

impl Into<String> for SetClockSource {
    fn into(self) -> String {
        let numeric_source: u8 = self.clock_source.into();
        format!("$CSS,{},{}", self.channel, numeric_source)
    }
}

impl SetClockSource {
    pub fn new(self, channel: u8, clock_source: ClockSource) -> Self {
        Self {
            channel,
            clock_source,
        }
    }
}

impl Default for SetClockSource {
    fn default() -> Self {
        Self {
            channel: 1,
            clock_source: ClockSource::Standalone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClockSource {
    Standalone,
    Master,
    Slave,
    SlaveInline,
}

impl ClockSource {
    /// 0 => Standalone
    /// 1 => Master
    /// 2 => Slave
    /// 3 => SlaveInline
    pub fn new(key: u8) -> Self {
        match key {
            0 => Self::Standalone,
            1 => Self::Master,
            2 => Self::Slave,
            3 => Self::SlaveInline,
            _ => Self::Standalone,
        }
    }
}

impl Into<u8> for ClockSource {
    fn into(self) -> u8 {
        match self {
            ClockSource::Standalone => 0,
            ClockSource::Master => 1,
            ClockSource::Slave => 2,
            ClockSource::SlaveInline => 3,
        }
    }
}

impl Default for ClockSource {
    fn default() -> Self {
        Self::Standalone
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetClockSourceResponse {
    /// The result of the command (Ok/Err)
    pub clock_source: ClockSource,
}

impl TryFrom<String> for GetClockSourceResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        // let clock_source = match parts[0] {
        //     Ok(clock_source) => clock_source,
        //     Err(_) => return Err(MWError::FailedParseResponse),
        // };

        // Hardcoding it for now - come back to fix later
        Ok(GetClockSourceResponse {
            clock_source: ClockSource::Standalone,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetClockSource {
    channel: u8,
}

impl Into<String> for GetClockSource {
    fn into(self) -> String {
        format!("$CSG,{}", self.channel)
    }
}

impl GetClockSource {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetClockSource {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
