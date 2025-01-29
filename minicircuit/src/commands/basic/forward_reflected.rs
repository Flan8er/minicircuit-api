use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Dbm, Watt},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerWattResponse {
    /// The forward power of the power amplifier in watts.
    pub forward: Watt,
    /// The reflected power of the power amplifier in watts.
    pub reflected: Watt,
}

impl TryFrom<String> for GetPAPowerWattResponse {
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

        let forward: Watt = match parts[2].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflected: Watt = match parts[3].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAPowerWattResponse { forward, reflected })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the forward and reflected power in watts.
pub struct GetPAPowerWatt {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAPowerWatt {
    fn into(self) -> String {
        format!("$PPG,{}", self.channel)
    }
}

impl GetPAPowerWatt {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerWatt {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerDBMResponse {
    /// The forward power of the power amplifier in dBm.
    pub forward: Dbm,
    /// The reflected power of the power amplifier in dBm.
    pub reflected: Dbm,
}

impl TryFrom<String> for GetPAPowerDBMResponse {
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

        let forward: Dbm = match parts[2].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflected: Dbm = match parts[3].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAPowerDBMResponse { forward, reflected })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the forward and reflected power of the power amplifier in dBm.
pub struct GetPAPowerDBM {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAPowerDBM {
    fn into(self) -> String {
        format!("$PPDG,{}", self.channel)
    }
}

impl GetPAPowerDBM {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerDBM {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
