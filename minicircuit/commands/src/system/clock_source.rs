use serde::{Deserialize, Serialize};

use crate::data_types::{errors::MWError, types::Channel};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetClockSourceResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetClockSourceResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetClockSourceResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Sets the clock source configuration (or "coherency mode") of the ISC board.
///
/// An ISC board can either use its own internal 10MHz Crystal Controlled Oscillator (XCO),
/// or it can accept an external clock reference from another ISC board.
/// The clock signal can be transmitted and received using a Low Voltage Differential Signaling (LVDS) transceiver.
///
/// The clock source is required to synchronize signal phase of ISC boards in
/// coherent multi-channel systems.
pub struct SetClockSource {
    pub channel: Channel,
    pub clock_source: ClockSource,
}

impl Into<String> for SetClockSource {
    fn into(self) -> String {
        let clock_source: u8 = self.clock_source.into();
        format!("$CSS,{},{}", self.channel, clock_source)
    }
}

impl SetClockSource {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel, clock_source: ClockSource) -> Self {
        Self {
            channel,
            clock_source,
        }
    }
}

impl Default for SetClockSource {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            clock_source: ClockSource::Standalone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetClockSourceResponse {
    /// Clock source configuration of the ISC board
    pub clock_source: ClockSource,
}

impl TryFrom<String> for GetClockSourceResponse {
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

        let clock_source: ClockSource = match parts[2].split('.').collect::<Vec<&str>>()[0]
            .trim()
            .parse::<u8>()
        {
            Ok(value) => ClockSource::new(value),
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetClockSourceResponse { clock_source })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the clock source configuration of the ISC board.
pub struct GetClockSource {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetClockSource {
    fn into(self) -> String {
        format!("$CSG,{}", self.channel)
    }
}

impl GetClockSource {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetClockSource {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Clock Source------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// 0 - Standalone
///
/// 1 - Master
///
/// 2 - Slave
///
/// 3 - Slave inline
pub enum ClockSource {
    /// Default.
    ///
    /// Use onboard XCO.
    ///
    /// Do not output reference signal.
    Standalone,
    /// Use onboard XCO.
    ///
    /// Output reference signal to slaves using LVDS.
    Master,
    /// Use external clock reference from LVDS.
    ///
    /// Do not output reference signal.
    Slave,
    /// Use external clock reference from LVDS.
    ///
    /// Output reference signal to slaves using LVDS.
    SlaveInline,
}
impl ClockSource {
    /// 0 => Standalone
    /// 1 => Master
    /// 2 => Slave
    /// 3 => SlaveInline
    pub fn new(key: u8) -> Self {
        match key {
            0 => Self::Standalone,
            1 => Self::Master,
            2 => Self::Slave,
            3 => Self::SlaveInline,
            _ => Self::Standalone,
        }
    }
}
impl Into<u8> for ClockSource {
    /// Converts a clock source variant into it's u8 equivalent.
    fn into(self) -> u8 {
        match self {
            ClockSource::Standalone => 0,
            ClockSource::Master => 1,
            ClockSource::Slave => 2,
            ClockSource::SlaveInline => 3,
        }
    }
}
impl Into<String> for ClockSource {
    fn into(self) -> String {
        match self {
            ClockSource::Standalone => String::from("standalone"),
            ClockSource::Master => String::from("master"),
            ClockSource::Slave => String::from("slave"),
            ClockSource::SlaveInline => String::from("slave-in-line"),
        }
    }
}
