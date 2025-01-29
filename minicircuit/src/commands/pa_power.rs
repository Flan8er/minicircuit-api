use serde::{Deserialize, Serialize};

use crate::{
    drivers::data_types::types::{Adc, Amperes, Channel, Dbm, Volts, Watt},
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerADCResponse {
    /// The forward power ADC count from 0 to 4095.
    pub forward: Adc,
    /// The reflected power ADC count from 0 to 4095.
    pub reflected: Adc,
}

impl TryFrom<String> for GetPAPowerADCResponse {
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

        let forward: Adc = match parts[2].trim().parse::<f32>() {
            Ok(value) => Adc::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let reflected: Adc = match parts[3].trim().parse::<f32>() {
            Ok(value) => Adc::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAPowerADCResponse { forward, reflected })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the forward and reflected power as ADC counts.
///
/// Depending on the PA Type, these ADC counts are either converted from the analog voltage inputs on the ISC board,
/// or from the ADCs on the ZHL-2425-250X+ (See `SetPAType`). If the source of the ADC count is the ISC board,
/// ADC measurements are averaged over 10 samples. Otherwise, a single sample is returned.
pub struct GetPAPowerADC {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAPowerADC {
    fn into(self) -> String {
        format!("$PAG,{}", self.channel)
    }
}

impl GetPAPowerADC {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerADC {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPAPowerSetpointWattResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPAPowerSetpointWattResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPAPowerSetpointWattResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the amplifier chain's output power setpoint to the desired value in watts.
pub struct SetPAPowerSetpointWatt {
    /// Channel identification number.
    pub channel: Channel,
    /// Desired power setpoint.
    pub power: Watt,
}

impl Into<String> for SetPAPowerSetpointWatt {
    fn into(self) -> String {
        format!("$PWRS,{},{}", self.channel, self.power)
    }
}

impl SetPAPowerSetpointWatt {
    /// Returns a handler to call the command.
    pub fn new(self, channel: Channel, power: Watt) -> Self {
        Self { channel, power }
    }
}

impl Default for SetPAPowerSetpointWatt {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            power: Watt::new(250.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerSetpointWattResponse {
    /// The current output power value for the RF signal in watt.
    pub power: Watt,
}

impl TryFrom<String> for GetPAPowerSetpointWattResponse {
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

        let power: Watt = match parts[2].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAPowerSetpointWattResponse { power })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the configured output power setpoint in watts.
pub struct GetPAPowerSetpointWatt {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAPowerSetpointWatt {
    fn into(self) -> String {
        format!("$PWRG,{}", self.channel)
    }
}

impl GetPAPowerSetpointWatt {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerSetpointWatt {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPAPowerSetpointDBMResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetPAPowerSetpointDBMResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetPAPowerSetpointDBMResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the output power setpoint to the desired value in dBm.
pub struct SetPAPowerSetpointDBM {
    /// Channel identification number.
    pub channel: Channel,
    /// Desired power value for the RF signal in dBm.
    pub power: Dbm,
}

impl Into<String> for SetPAPowerSetpointDBM {
    fn into(self) -> String {
        format!("$PWRDS,{},{}", self.channel, self.power)
    }
}

impl SetPAPowerSetpointDBM {
    /// Returns a handler to call the command.
    pub fn new(self, channel: Channel, power: Dbm) -> Self {
        Self { channel, power }
    }
}

impl Default for SetPAPowerSetpointDBM {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            power: Dbm::new(50.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAPowerSetpointDBMResponse {
    /// The current power value for the RF signal in dBm.
    pub power: Dbm,
}

impl TryFrom<String> for GetPAPowerSetpointDBMResponse {
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

        let power: Dbm = match parts[2].trim().parse::<f32>() {
            Ok(value) => Dbm::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAPowerSetpointDBMResponse { power })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the configured output power setpoint in dBm.
pub struct GetPAPowerSetpointDBM {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAPowerSetpointDBM {
    fn into(self) -> String {
        format!("$PWRDG,{}", self.channel)
    }
}

impl GetPAPowerSetpointDBM {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAPowerSetpointDBM {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPACurrentResponse {
    /// DC current readings of the ISC in Amps.
    pub current: Amperes,
}

impl TryFrom<String> for GetPACurrentResponse {
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

        let current: Amperes = match parts[2].trim().parse::<f32>() {
            Ok(value) => Amperes::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPACurrentResponse { current })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the DC current reading of the ISC in Amps.
pub struct GetPACurrent {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPACurrent {
    fn into(self) -> String {
        format!("$PIG,{}", self.channel)
    }
}

impl GetPACurrent {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPACurrent {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPAVoltageResponse {
    /// Measured DC voltage of the PA in Volts.
    pub voltage: Volts,
}

impl TryFrom<String> for GetPAVoltageResponse {
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

        let voltage: Volts = match parts[2].trim().parse::<f32>() {
            Ok(value) => Volts::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAVoltageResponse { voltage })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Returns the measured DC voltage of the PA in Volts.
pub struct GetPAVoltage {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAVoltage {
    fn into(self) -> String {
        format!("$PVG,{}", self.channel)
    }
}

impl GetPAVoltage {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAVoltage {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
