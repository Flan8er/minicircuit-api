use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Channel, Frequency, MainDelay, Threshold},
    errors::MWError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetDLLModeResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetDLLModeResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetDLLModeResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Turns DLL mode ON or OFF
///
/// True = On,
/// False = Off
pub struct SetDLLMode {
    /// Channel identification number.
    pub channel: Channel,
    /// The desired enable state of the DLL mode. Defaults to Off
    ///
    /// True = On
    ///
    /// False = Off
    pub enabled: bool,
}

impl Into<String> for SetDLLMode {
    fn into(self) -> String {
        let numeric_value = match self.enabled {
            true => 1,
            false => 0,
        };
        format!("$DLES,{},{}", self.channel, numeric_value)
    }
}

impl SetDLLMode {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(self, channel: Channel, enabled: bool) -> Self {
        Self { channel, enabled }
    }
}

impl Default for SetDLLMode {
    /// Returns the default handler to call the command.
    /// By default DLL is enabled.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetDLLModeResponse {
    /// Whether the DLL mode is currently turned ON or OFF
    pub enabled: bool,
}

impl TryFrom<String> for GetDLLModeResponse {
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

        let enabled: bool = match parts[2].trim().parse::<u8>() {
            Ok(value) => match value {
                1 => true,
                _ => false,
            },
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetDLLModeResponse { enabled })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the state of DLL mode - either turned ON or OFF
pub struct GetDLLMode {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetDLLMode {
    fn into(self) -> String {
        format!("$DLEG,{}", self.channel)
    }
}

impl GetDLLMode {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetDLLMode {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetDLLConfigResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetDLLConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetDLLConfigResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetDLLConfig {
    /// Channel identification number.
    pub channel: Channel,
    /// The lower boundary of the bandwidth for DLL in MHz.
    pub lower_frequency: Frequency,
    /// The upper boundary of the bandwidth for DLL in MHz.
    pub upper_frequency: Frequency,
    /// The frequency at which the DLL starts it's activities in MHz.
    pub start_frequency: Frequency,
    /// The step size of the DLL in MHz.
    pub step_frequency: Frequency,
    /// The match/efficiency threshold in dB to be met before DLL latches onto a frequency.
    pub threshold: Threshold,
    /// The delay between complete runs of the DLL in ms.
    pub main_delay: MainDelay,
}

impl Into<String> for SetDLLConfig {
    fn into(self) -> String {
        format!(
            "$DLCS,{},{},{},{},{},{},{}",
            self.channel,
            self.lower_frequency,
            self.upper_frequency,
            self.start_frequency,
            self.step_frequency,
            self.threshold,
            self.main_delay
        )
    }
}

impl SetDLLConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(
        self,
        channel: Channel,
        lower_frequency: Frequency,
        upper_frequency: Frequency,
        start_frequency: Frequency,
        step_frequency: Frequency,
        threshold: Threshold,
        main_delay: MainDelay,
    ) -> Self {
        Self {
            channel,
            lower_frequency,
            upper_frequency,
            start_frequency,
            step_frequency,
            threshold,
            main_delay,
        }
    }
}

impl Default for SetDLLConfig {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            lower_frequency: Frequency::new(2400),
            upper_frequency: Frequency::new(2500),
            start_frequency: Frequency::new(2410),
            step_frequency: Frequency::new(5),
            threshold: Threshold::new(0.5),
            main_delay: MainDelay::new(25),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetDLLConfigResponse {
    /// The lower boundary of the bandwidth for DLL in MHz.
    pub lower_frequency: Frequency,
    /// The upper boundary of the bandwidth for DLL in MHz.
    pub upper_frequency: Frequency,
    /// The frequency at which the DLL starts it's activities in MHz.
    pub start_frequency: Frequency,
    /// The step size of the DLL in MHz.
    pub step_frequency: Frequency,
    /// The match/efficiency threshold in dB to be met before DLL latches onto a frequency.
    pub threshold: Threshold,
    /// The delay between complete runs of the DLL in ms.
    pub main_delay: MainDelay,
}

impl TryFrom<String> for GetDLLConfigResponse {
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
        if parts.len() != 5 {
            return Err(Self::Error::FailedParseResponse);
        }

        let lower_frequency: Frequency = match parts[2].trim().parse::<u16>() {
            Ok(value) => Frequency::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let upper_frequency: Frequency = match parts[3].trim().parse::<u16>() {
            Ok(value) => Frequency::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let start_frequency: Frequency = match parts[4].trim().parse::<u16>() {
            Ok(value) => Frequency::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let step_frequency: Frequency = match parts[4].trim().parse::<u16>() {
            Ok(value) => Frequency::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let threshold: Threshold = match parts[4].trim().parse::<f32>() {
            Ok(value) => Threshold::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let main_delay: MainDelay = match parts[4].trim().parse::<u16>() {
            Ok(value) => MainDelay::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetDLLConfigResponse {
            lower_frequency,
            upper_frequency,
            start_frequency,
            step_frequency,
            threshold,
            main_delay,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetDLLConfig {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetDLLConfig {
    fn into(self) -> String {
        format!("$DLCG,{}", self.channel)
    }
}

impl GetDLLConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetDLLConfig {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
