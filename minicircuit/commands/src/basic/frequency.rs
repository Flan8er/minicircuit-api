use serde::{Deserialize, Serialize};

use crate::data_types::{
    errors::MWError,
    types::{Channel, Frequency},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetFrequencyResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetFrequencyResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetFrequencyResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Sets the frequecy of the ISC board's RF output to the desired value in MHz.
pub struct SetFrequency {
    /// Channel identification number.
    pub channel: Channel,
    /// Desired frequency setting for the RF signal.
    pub frequency: Frequency,
}

impl Into<String> for SetFrequency {
    fn into(self) -> String {
        format!("$FCS,{},{}", self.channel, self.frequency)
    }
}

impl SetFrequency {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(channel: Channel, frequency: Frequency) -> Self {
        Self { channel, frequency }
    }
}

impl Default for SetFrequency {
    /// Returns the default handler to call the command.
    ///
    /// By default, frequency is set to 2450MHz.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            frequency: Frequency::new(2450),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetFrequencyResponse {
    /// Current frequency setting of the ISC board (in MHz).
    pub frequency: Frequency,
}

impl TryFrom<String> for GetFrequencyResponse {
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

        let frequency: Frequency = match parts[2].split('.').collect::<Vec<&str>>()[0]
            .trim()
            .parse::<u16>()
        {
            Ok(value) => Frequency::new(value),
            Err(_) => return Err(Self::Error::FailedParseResponse),
        };

        Ok(GetFrequencyResponse { frequency })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the frequency of the ISC board's RF output in MHz.
pub struct GetFrequency {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetFrequency {
    fn into(self) -> String {
        format!("$FCG,{}", self.channel)
    }
}

impl GetFrequency {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetFrequency {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
