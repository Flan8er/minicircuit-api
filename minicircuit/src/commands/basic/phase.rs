use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Phase},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPhaseResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPhaseResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPhaseResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Sets the phase of the ISC board's RF output in degrees.
///
/// The phase set is reference to the selected clock source (see ClockSource).
pub struct SetPhase {
    /// Channel identification number.
    pub channel: Channel,
    /// The desired phase value in degrees
    ///
    /// Valid values are between 0 and 359.
    pub phase: Phase,
}

impl Into<String> for SetPhase {
    fn into(self) -> String {
        format!("$PCS,{},{}", self.channel, self.phase)
    }
}

impl SetPhase {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(self, channel: Channel, phase: Phase) -> Self {
        Self { channel, phase }
    }
}

impl Default for SetPhase {
    /// Returns the default handler to call the command.
    ///
    /// By default, phase is set to 0 degrees.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            phase: Phase::new(0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetPhaseResponse {
    /// Current phase value of the ISC board (in degrees).
    pub phase: Phase,
}

impl TryFrom<String> for GetPhaseResponse {
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

        let phase: Phase = match parts[2].split('.').collect::<Vec<&str>>()[0]
            .trim()
            .parse::<u16>()
        {
            Ok(value) => Phase::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPhaseResponse { phase })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the current phase value of the ISC board's RF output in degrees.
pub struct GetPhase {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPhase {
    fn into(self) -> String {
        format!("$PCG,{}", self.channel)
    }
}

impl GetPhase {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPhase {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
