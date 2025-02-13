use serde::{Deserialize, Serialize};

use crate::data_types::{errors::MWError, types::Channel};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetDLLEnabledResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetDLLEnabledResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetDLLEnabledResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Turns DLL mode ON or OFF
///
/// True = On,
/// False = Off (default)
pub struct SetDLLEnabled {
    /// Channel identification number.
    pub channel: Channel,
    /// The desired enable state of the DLL mode. Defaults to Off
    ///
    /// True = On
    ///
    /// False = Off
    pub enabled: bool,
}

impl Into<String> for SetDLLEnabled {
    fn into(self) -> String {
        let numeric_value = match self.enabled {
            true => 1,
            false => 0,
        };
        format!("$DLES,{},{}", self.channel, numeric_value)
    }
}

impl SetDLLEnabled {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(channel: Channel, enabled: bool) -> Self {
        Self { channel, enabled }
    }
}

impl Default for SetDLLEnabled {
    /// Returns the default handler to call the command.
    /// By default DLL is disabled.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            enabled: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetDLLEnabledResponse {
    /// Whether the DLL mode is currently turned ON or OFF
    pub enabled: bool,
}

impl TryFrom<String> for GetDLLEnabledResponse {
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

        let enabled: bool = match parts[2].split('.').collect::<Vec<&str>>()[0]
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

        Ok(GetDLLEnabledResponse { enabled })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the state of DLL mode - either turned ON or OFF
pub struct GetDLLEnabled {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetDLLEnabled {
    fn into(self) -> String {
        format!("$DLEG,{}", self.channel)
    }
}

impl GetDLLEnabled {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetDLLEnabled {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
