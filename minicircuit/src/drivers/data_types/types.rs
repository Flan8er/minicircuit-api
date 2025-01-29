use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Frequency---------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Frequency {
    /// Typical values are in MHz.
    pub frequency: u16,
}
impl Frequency {
    /// Creates a new frequency operator in units of MHz.
    pub fn new(frequency: u16) -> Self {
        Self { frequency }
    }
}
impl Into<u16> for Frequency {
    fn into(self) -> u16 {
        self.frequency
    }
}
impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.frequency)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ---------------------------Channel----------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Channel {
    pub channel_id: u8,
}
impl Channel {
    pub fn new(channel_id: u8) -> Self {
        Self { channel_id }
    }
}
impl Default for Channel {
    fn default() -> Self {
        Self { channel_id: 1 }
    }
}
impl Into<u8> for Channel {
    fn into(self) -> u8 {
        self.channel_id
    }
}
impl Display for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.channel_id)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// -----------------------------Watt------------------------------ //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Watt {
    pub power: f32,
}
impl Watt {
    pub fn new(power: f32) -> Self {
        Self { power }
    }
}
impl Into<f32> for Watt {
    fn into(self) -> f32 {
        self.power
    }
}
impl Display for Watt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}", self.power)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ------------------------------dBm------------------------------ //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Dbm {
    pub power: f32,
}
impl Dbm {
    pub fn new(power: f32) -> Self {
        Self { power }
    }
}
impl Into<f32> for Dbm {
    fn into(self) -> f32 {
        self.power
    }
}
impl Display for Dbm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}", self.power)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ------------------------------ADC------------------------------ //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Adc {
    pub power: f32,
}
impl Adc {
    pub fn new(power: f32) -> Self {
        Self {
            power: power.clamp(0., 4095.),
        }
    }
}
impl Into<f32> for Adc {
    fn into(self) -> f32 {
        self.power
    }
}
impl Display for Adc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}", self.power)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ----------------------------Amperes---------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Amperes {
    pub current: f32,
}
impl Amperes {
    pub fn new(current: f32) -> Self {
        Self { current }
    }
}
impl Into<f32> for Amperes {
    fn into(self) -> f32 {
        self.current
    }
}
impl Display for Amperes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}", self.current)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// -----------------------------Volts----------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Volts {
    pub voltage: f32,
}
impl Volts {
    pub fn new(voltage: f32) -> Self {
        Self { voltage }
    }
}
impl Into<f32> for Volts {
    fn into(self) -> f32 {
        self.voltage
    }
}
impl Display for Volts {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}", self.voltage)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Temperature-------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Units of °C.
pub struct Temperature {
    pub temperature: u8,
}
impl Temperature {
    pub fn new(temperature: u8) -> Self {
        Self { temperature }
    }
}
impl Into<u8> for Temperature {
    fn into(self) -> u8 {
        self.temperature
    }
}
impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.temperature)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ----------------------------Seconds---------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Seconds {
    pub seconds: u64,
}
impl Seconds {
    pub fn new(seconds: u64) -> Self {
        Self { seconds }
    }
}
impl Into<u64> for Seconds {
    fn into(self) -> u64 {
        self.seconds
    }
}
impl Display for Seconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.seconds)
    }
}
impl Default for Seconds {
    fn default() -> Self {
        Self { seconds: 0 }
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Clock Source------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
impl Display for ClockSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let source: u8 = self.to_owned().into();
        write!(f, "{}", source)
    }
}
impl Default for ClockSource {
    fn default() -> Self {
        Self::Standalone
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ----------------------------Phase------------------------------ //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Phase {
    /// Values are in degrees.
    pub phase: u16,
}
impl Phase {
    /// Creates a new phase operator in units of degrees.
    ///
    /// Valid values are between 0 and 359.
    pub fn new(phase: u16) -> Self {
        Self {
            phase: phase.clamp(0, 359),
        }
    }
}
impl Into<u16> for Phase {
    fn into(self) -> u16 {
        self.phase
    }
}
impl Display for Phase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.phase)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// -------------------------Attenuation--------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Attenuation {
    /// Values are in dB.
    pub attenuation: f32,
}
impl Attenuation {
    /// Creates a new phase operator in units of dB.
    ///
    /// Valid values are between 0 - 31.5 dB.
    ///
    /// Minimum step size: 0.25 dB.
    pub fn new(attenuation: f32) -> Self {
        let clamped = attenuation.clamp(0.0, 31.5);
        let rounded = (clamped / 0.25).round() * 0.25; // Round to the nearest 0.5
        Self {
            attenuation: rounded,
        }
    }
}
impl Into<f32> for Attenuation {
    fn into(self) -> f32 {
        self.attenuation
    }
}
impl Display for Attenuation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.1}", self.attenuation)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Percentage--------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Percentage {
    pub percentage: u8,
}
impl Percentage {
    pub fn new(percentage: u8) -> Self {
        Self {
            percentage: percentage.clamp(0, 100),
        }
    }
}
impl Into<u8> for Percentage {
    fn into(self) -> u8 {
        self.percentage
    }
}
impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.percentage)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// -----------------------Correction Factor----------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CorrectionFactor {
    pub correction_factor: u8,
}
impl CorrectionFactor {
    pub fn new(correction_factor: u8) -> Self {
        Self { correction_factor }
    }
}
impl Into<u8> for CorrectionFactor {
    fn into(self) -> u8 {
        self.correction_factor
    }
}
impl std::fmt::Display for CorrectionFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.correction_factor)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Main Delay--------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MainDelay {
    pub main_delay: u16,
}
impl MainDelay {
    pub fn new(main_delay: u16) -> Self {
        Self { main_delay }
    }
}
impl Into<u16> for MainDelay {
    fn into(self) -> u16 {
        self.main_delay
    }
}
impl std::fmt::Display for MainDelay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.main_delay)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Threshold---------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Threshold {
    pub threshold: f32,
}
impl Threshold {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }
}
impl Into<f32> for Threshold {
    fn into(self) -> f32 {
        self.threshold
    }
}
impl std::fmt::Display for Threshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.threshold)
    }
}
