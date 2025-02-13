use serde::{Deserialize, Serialize};

use crate::data_types::{
    errors::MWError,
    types::{Channel, Watt},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOADissipationConfigResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOADissipationConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOADissipationConfigResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Sets the dissipation at which SOA takes action in Watts.
///
/// One of the features of the SOA is protection against excessive power dissipation inside a generator.
/// Excessive power dissipation occurs when an RF system draws a disproportionate amount of current from it's
/// power supply (PSU) relative to the amount RF energy that is transmitted into a load. High dissipation
/// can be reached when the system is poorly matched or when the system is well matched but still operating
/// with poor efficiency. At the system level, dissipation is the rate that heat needs to be removed from the
/// generator by means of heat sink or cooling plate to maintain a stable temperature. The dissipation SOA
/// could be used in systems with limited cooling capacity to issue a warning to the user to shut the generator
/// down before it has a change to heat up to the temperature shutdown limit.
pub struct SetSOADissipationConfig {
    /// Channel identification number.
    pub channel: Channel,
    /// The dissipation value in W at which the `HighDissipation` reaction is performed by the SOA.
    pub high_dissipation: Watt,
    /// The dissipation value in W at which the `ShutdownDissipation` reaction is performed by the SOA.
    pub shutdown_dissipation: Watt,
}

impl Into<String> for SetSOADissipationConfig {
    fn into(self) -> String {
        format!(
            "$SDS,{},{},{}",
            self.channel, self.high_dissipation, self.shutdown_dissipation
        )
    }
}

impl SetSOADissipationConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(channel: Channel, high_dissipation: Watt, shutdown_dissipation: Watt) -> Self {
        Self {
            channel,
            high_dissipation,
            shutdown_dissipation,
        }
    }
}

impl Default for SetSOADissipationConfig {
    /// Returns the default handler to call the command.
    /// By default, protection values are both configured to 0W.
    /// Since this SOA is not enabled by default, these values have no effect on the system operation.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            high_dissipation: Watt::new(0.),
            shutdown_dissipation: Watt::new(0.),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSOADissipationConfigResponse {
    /// The dissipation value in W at which the `HighDissipation` reaction is performed by the SOA.
    pub high_dissipation: Watt,
    /// The dissipation value in W at which the `ShutdownDissipation` reaction is performed by the SOA.
    pub shutdown_dissipation: Watt,
}

impl TryFrom<String> for GetSOADissipationConfigResponse {
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

        let high_dissipation: Watt = match parts[2].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };
        let shutdown_dissipation: Watt = match parts[3].trim().parse::<f32>() {
            Ok(value) => Watt::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetSOADissipationConfigResponse {
            high_dissipation,
            shutdown_dissipation,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the dissipation at which SOA takes action in Watts.
///
/// One of the features of the SOA is protection against excessive power dissipation inside a generator.
/// Excessive power dissipation occurs when an RF system draws a disproportionate amount of current from it's
/// power supply (PSU) relative to the amount RF energy that is transmitted into a load. High dissipation
/// can be reached when the system is poorly matched or when the system is well matched but still operating
/// with poor efficiency. At the system level, dissipation is the rate that heat needs to be removed from the
/// generator by means of heat sink or cooling plate to maintain a stable temperature. The dissipation SOA
/// could be used in systems with limited cooling capacity to issue a warning to the user to shut the generator
/// down before it has a change to heat up to the temperature shutdown limit.
pub struct GetSOADissipationConfig {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetSOADissipationConfig {
    fn into(self) -> String {
        format!("$SDG,{}", self.channel)
    }
}

impl GetSOADissipationConfig {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetSOADissipationConfig {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
