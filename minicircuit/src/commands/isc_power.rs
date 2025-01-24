use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetISCPowerOutputResponse {
    /// The uptime in seconds.
    pub result: String,
}

impl TryFrom<String> for SetISCPowerOutputResponse {
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

        Ok(SetISCPowerOutputResponse { result })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetISCPowerOutput {
    channel: u8,
    power_dbm: f32,
}

impl Into<String> for SetISCPowerOutput {
    fn into(self) -> String {
        format!("$PWRSGDS,{},{}", self.channel, self.power_dbm)
    }
}

impl SetISCPowerOutput {
    pub fn new(self, channel: u8, power_dbm: f32) -> Self {
        Self { channel, power_dbm }
    }
}

impl Default for SetISCPowerOutput {
    fn default() -> Self {
        Self {
            channel: 1,
            power_dbm: 20.,
        }
    }
}
