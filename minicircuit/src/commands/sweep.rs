use serde::{Deserialize, Serialize};

use crate::drivers::data_types::types::{Channel, Dbm, Frequency, Watt};
use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerformSweepWattResponse {
    pub start_frequency: Frequency,
    pub stop_frequency: Frequency,
    pub step_frequency: Frequency,
    pub power_watt: Watt,
}

impl TryFrom<String> for PerformSweepWattResponse {
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

        Ok(PerformSweepWattResponse {
            start_frequency: todo!(),
            stop_frequency: todo!(),
            step_frequency: todo!(),
            power_watt: todo!(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerformSweepWatt {
    channel: Channel,
    start_frequency: Frequency,
    stop_frequency: Frequency,
}

impl Into<String> for PerformSweepWatt {
    fn into(self) -> String {
        format!(
            "$SWP,{},{},{},0",
            Into::<u8>::into(self.channel),
            Into::<u16>::into(self.start_frequency),
            Into::<u16>::into(self.stop_frequency)
        )
    }
}

impl PerformSweepWatt {
    pub fn new(
        self,
        channel: Channel,
        start_frequency: Frequency,
        stop_frequency: Frequency,
    ) -> Self {
        Self {
            channel,
            start_frequency,
            stop_frequency,
        }
    }
}

impl Default for PerformSweepWatt {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            start_frequency: Frequency::new(2400),
            stop_frequency: Frequency::new(2500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerformSweepDBMResponse {
    pub start_frequency: Frequency,
    pub stop_frequency: Frequency,
    pub step_frequency: Frequency,
    pub power_watt: Dbm,
}

impl TryFrom<String> for PerformSweepDBMResponse {
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

        Ok(PerformSweepDBMResponse {
            start_frequency: todo!(),
            stop_frequency: todo!(),
            step_frequency: todo!(),
            power_watt: todo!(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerformSweepDBM {
    channel: Channel,
    start_frequency: Frequency,
    stop_frequency: Frequency,
}

impl Into<String> for PerformSweepDBM {
    fn into(self) -> String {
        format!(
            "$SWP,{},{},{},0",
            Into::<u8>::into(self.channel),
            Into::<u16>::into(self.start_frequency),
            Into::<u16>::into(self.stop_frequency)
        )
    }
}

impl PerformSweepDBM {
    pub fn new(
        self,
        channel: Channel,
        start_frequency: Frequency,
        stop_frequency: Frequency,
    ) -> Self {
        Self {
            channel,
            start_frequency,
            stop_frequency,
        }
    }
}

impl Default for PerformSweepDBM {
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            start_frequency: Frequency::new(2400),
            stop_frequency: Frequency::new(2500),
        }
    }
}
