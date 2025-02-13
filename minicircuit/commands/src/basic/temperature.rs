use serde::{Deserialize, Serialize};

use crate::data_types::{
    errors::MWError,
    types::{Channel, Temperature},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetPATempResponse {
    /// The temperature of the power amplifier (PA).
    pub temperature: Temperature,
}

impl TryFrom<String> for GetPATempResponse {
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
        if parts.len() != 3 {
            return Err(Self::Error::FailedParseResponse);
        }

        let temperature: Temperature = match parts[2].split('.').collect::<Vec<&str>>()[0]
            .trim()
            .parse::<u8>()
        {
            Ok(value) => Temperature::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPATempResponse { temperature })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the temperature of the power amplifier (PA).
pub struct GetPATemp {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPATemp {
    fn into(self) -> String {
        format!("$PTG,{}", self.channel)
    }
}

impl GetPATemp {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPATemp {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
