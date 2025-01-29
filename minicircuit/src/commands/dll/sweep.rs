use serde::{Deserialize, Serialize};

use crate::drivers::data_types::types::{Channel, Dbm, Frequency, Watt};
use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// The best frequency to be at given the requested power output.
pub struct PerformSweepWattResponse {
    /// The frequency at which the best result occurred.
    pub measurement_frequency: Frequency,
    /// The forward power measured at the measured frequency.
    pub forward_power: Watt,
    /// The reflected power measured at the measured frequency.
    pub reflected_power: Watt,
}

impl TryFrom<String> for PerformSweepWattResponse {
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

        let measurement_frequency: Frequency = match parts[2].trim().parse::<u16>() {
            Ok(value) => Frequency::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let forward_power: Watt = match parts[3].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflected_power: Watt = match parts[4].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(PerformSweepWattResponse {
            measurement_frequency,
            forward_power,
            reflected_power,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Output's the best frequency to be at given the requested power output.
///
/// Performs an S11 frequency sweep across the band provided.
///
/// The completion time of the command will increase as the number of frequency steps increases.
/// This can make it seem as if the ISC board has become un-responsive for some time.
pub struct PerformSweepWatt {
    /// Channel identification number.
    pub channel: Channel,
    /// The beginning of the sweep bandwidth in MHz.
    pub start_frequency: Frequency,
    /// The end of the sweep bandwidth in MHz.
    pub stop_frequency: Frequency,
    /// The size of the steps taken between each measurement in MHz.
    pub step_frequency: Frequency,
    /// The output power at which the sweep is performed in watts.
    pub power: Watt,
}

impl Into<String> for PerformSweepWatt {
    fn into(self) -> String {
        format!(
            "$SWP,{},{},{},{},{},1",
            self.channel,
            self.start_frequency,
            self.stop_frequency,
            self.step_frequency,
            self.power
        )
    }
}

impl PerformSweepWatt {
    /// Returns a handler to call the command using the given inputs.
    ///
    /// Channel identification number.
    ///
    /// The beginning of the sweep bandwidth in MHz.
    ///
    /// The end of the sweep bandwidth in MHz.
    ///
    /// The size of the steps taken between each measurement in MHz.
    ///
    /// The output power at which the sweep is performed in watts.
    pub fn new(
        self,
        channel: Channel,
        start_frequency: Frequency,
        stop_frequency: Frequency,
        step_frequency: Frequency,
        power: Watt,
    ) -> Self {
        Self {
            channel,
            start_frequency,
            stop_frequency,
            step_frequency,
            power,
        }
    }
}

impl Default for PerformSweepWatt {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            start_frequency: Frequency::new(2400),
            stop_frequency: Frequency::new(2500),
            step_frequency: Frequency::new(10),
            power: Watt::new(100.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// The best frequency to be at given the requested power output.
pub struct PerformSweepDBMResponse {
    /// The frequency at which the best result occurred.
    pub measurement_frequency: Frequency,
    /// The forward power measured at the measured frequency.
    pub forward_power: Dbm,
    /// The reflected power measured at the measured frequency.
    pub reflected_power: Dbm,
}

impl TryFrom<String> for PerformSweepDBMResponse {
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

        let measurement_frequency: Frequency = match parts[2].trim().parse::<u16>() {
            Ok(value) => Frequency::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let forward_power: Dbm = match parts[3].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflected_power: Dbm = match parts[4].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(PerformSweepDBMResponse {
            measurement_frequency,
            forward_power,
            reflected_power,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Output's the best frequency to be at given the requested power output.
///
/// Performs an S11 frequency sweep across the band provided.
///
/// The completion time of the command will increase as the number of frequency steps increases.
/// This can make it seem as if the ISC board has become un-responsive for some time.
pub struct PerformSweepDBM {
    /// Channel identification number.
    pub channel: Channel,
    /// The beginning of the sweep bandwidth in MHz.
    pub start_frequency: Frequency,
    /// The end of the sweep bandwidth in MHz.
    pub stop_frequency: Frequency,
    /// The size of the steps taken between each measurement in MHz.
    pub step_frequency: Frequency,
    /// The output power at which the sweep is performed in dBm.
    pub power: Dbm,
}

impl Into<String> for PerformSweepDBM {
    fn into(self) -> String {
        format!(
            "$SWPD,{},{},{},{},{},1",
            self.channel,
            self.start_frequency,
            self.stop_frequency,
            self.step_frequency,
            self.power
        )
    }
}

impl PerformSweepDBM {
    /// Returns a handler to call the command using the given inputs.
    ///
    /// Channel identification number.
    ///
    /// The beginning of the sweep bandwidth in MHz.
    ///
    /// The end of the sweep bandwidth in MHz.
    ///
    /// The size of the steps taken between each measurement in MHz.
    ///
    /// The output power at which the sweep is performed in dBm.
    pub fn new(
        self,
        channel: Channel,
        start_frequency: Frequency,
        stop_frequency: Frequency,
        step_frequency: Frequency,
        power: Dbm,
    ) -> Self {
        Self {
            channel,
            start_frequency,
            stop_frequency,
            step_frequency,
            power,
        }
    }
}

impl Default for PerformSweepDBM {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            start_frequency: Frequency::new(2400),
            stop_frequency: Frequency::new(2500),
            step_frequency: Frequency::new(10),
            power: Dbm::new(10.),
        }
    }
}
