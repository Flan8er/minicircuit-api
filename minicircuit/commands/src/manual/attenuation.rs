use serde::{Deserialize, Serialize};

use crate::data_types::{
    errors::MWError,
    types::{Attenuation, Channel},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// The configured attenuation value of the VGA which regulates the ISC board’s power output. The
/// higher the value, the lower the power output.
pub struct GetAttenuationResponse {
    /// The attenuation value of the DSA.
    ///
    /// Attenuation range: 0 - 31.75dB
    ///
    /// Minimum step size: 0.25dB
    pub attenuation: Attenuation,
}

impl TryFrom<String> for GetAttenuationResponse {
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

        let attenuation: Attenuation = match parts[2].trim().parse::<f32>() {
            Ok(value) => Attenuation::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetAttenuationResponse { attenuation })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the configured attenuation value of the VGA which regulates the ISC board's power output.
/// The higher the value, the lower the power output.
pub struct GetAttenuation {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetAttenuation {
    fn into(self) -> String {
        format!("$GCG,{}", self.channel)
    }
}

impl GetAttenuation {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetAttenuation {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetAttenuationResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetAttenuationResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetAttenuationResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// TO USE THIS COMMAND, `SetAutoGain` MUST BE DISABLED FIRST
///
/// Set the attenuation of the variable gain amplifier (VGA) which regulates
/// the ISC board's power output.
/// The higher the value, the lower the power output.
///
/// Under normal conditions, both the VGA and the IQ modulator are used to regulate power output of the ISC board,
/// thus the actual power output is a combination of both.
/// The IQ modulator is controlled using the SetQIMagPercent command.
pub struct SetAttenuation {
    /// Channel identification number.
    pub channel: Channel,
    /// The desired attenuation value of the DSA.
    ///
    /// Attenuation range: 0 - 31.75dB
    ///
    /// Minimum step size: 0.25dB
    pub attenuation: Attenuation,
}

impl Into<String> for SetAttenuation {
    fn into(self) -> String {
        format!("$GCS,{},{}", self.channel, self.attenuation)
    }
}

impl SetAttenuation {
    /// Returns a handler to call the command with the provided inputs.
    pub fn new(channel: Channel, attenuation: Attenuation) -> Self {
        Self {
            channel,
            attenuation,
        }
    }
}

impl Default for SetAttenuation {
    /// Returns the default handler to call the command.
    ///
    /// By default, attenuation is set to 7dB
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            attenuation: Attenuation::new(7.),
        }
    }
}
