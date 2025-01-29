use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, CorrectionFactor, Frequency, Percentage},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPWMDutyCycleResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPWMDutyCycleResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPWMDutyCycleResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the PWM duty cycle between 0% and 100%.
///
/// This command doubles as a PWM ON/OFF switch. Setting the duty cycle
/// to 100% is the same as turning PWN off entirely, thus there is no
/// dedicated PWM ON/OFF command.
pub struct SetPWMDutyCycle {
    /// Channel identification number.
    pub channel: Channel,
    /// A value between 0 and 100 that sets the duty cycle in percent.
    pub duty_cycle: Percentage,
}

impl Into<String> for SetPWMDutyCycle {
    fn into(self) -> String {
        format!("$DCS,{},{}", self.channel, self.duty_cycle)
    }
}

impl SetPWMDutyCycle {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(self, channel: Channel, duty_cycle: Percentage) -> Self {
        Self {
            channel,
            duty_cycle,
        }
    }
}

impl Default for SetPWMDutyCycle {
    /// Returns the default handler to call the command.
    /// By default, duty cycle is set to 100%.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            duty_cycle: Percentage::new(100),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPWMSettingsResponse {
    /// The current PWM frequency.
    pub frequency: Frequency,
    /// The current correction factor for the PWM signal.
    pub correction_factor: CorrectionFactor,
    /// The current duty cycle percentage value.
    pub duty_cycle: Percentage,
}

impl TryFrom<String> for GetPWMSettingsResponse {
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
        if parts.len() != 11 {
            return Err(Self::Error::FailedParseResponse);
        }

        let frequency: Frequency = match parts[2].trim().parse::<u16>() {
            Ok(value) => Frequency::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let correction_factor: CorrectionFactor = match parts[3].trim().parse::<u8>() {
            Ok(value) => CorrectionFactor::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let duty_cycle: Percentage = match parts[10].trim().parse::<u8>() {
            Ok(value) => Percentage::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPWMSettingsResponse {
            frequency,
            correction_factor,
            duty_cycle,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns all the settings relating to PWM.
pub struct GetPWMSettings {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPWMSettings {
    fn into(self) -> String {
        format!("$DCG,{}", self.channel)
    }
}

impl GetPWMSettings {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPWMSettings {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetTimedRFEnableResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetTimedRFEnableResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetTimedRFEnableResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Initiates a single timed enable of specified duration.
pub struct SetTimedRFEnable {
    /// Channel identification number.
    pub channel: Channel,
    /// Duration of the timed enable in microseconds.
    pub duration: u32,
}

impl Into<String> for SetTimedRFEnable {
    fn into(self) -> String {
        format!("$ECST,{},1,{}", self.channel, self.duration)
    }
}

impl SetTimedRFEnable {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(self, channel: Channel, duration: u32) -> Self {
        Self { channel, duration }
    }
}

impl Default for SetTimedRFEnable {
    /// Returns the default handler to call the command.
    /// By default, duration is set to 5,000,000 microseconds (5 seconds).
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            duration: 5_000_000,
        }
    }
}
