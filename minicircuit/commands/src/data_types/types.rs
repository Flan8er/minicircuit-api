#[cfg(feature = "stores")]
use reactive_stores::Patch;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Frequency---------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
#[cfg_attr(feature = "stores", derive(Patch))]
pub struct Frequency {
    /// Typical values are in MHz.
    pub frequency: u16,
}

impl Frequency {
    pub fn new(frequency: u16) -> Self {
        Self { frequency }
    }
}

impl FromStr for Frequency {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim().parse::<u16>() {
            Ok(num) => Ok(Frequency::new(num)),
            Err(_) => Err(format!("Invalid frequency format: '{}'", s)),
        }
    }
}

impl From<Frequency> for u16 {
    fn from(f: Frequency) -> u16 {
        f.frequency
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.frequency)
    }
}

impl Add for Frequency {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Frequency::new(self.frequency.wrapping_add(other.frequency))
    }
}

impl Sub for Frequency {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Frequency::new(self.frequency.wrapping_sub(other.frequency))
    }
}

impl From<u16> for Frequency {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ---------------------------Channel----------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "stores", derive(Patch))]
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
impl From<Dbm> for Watt {
    fn from(dbm_value: Dbm) -> Watt {
        let dbm_value: f32 = dbm_value.into();

        let converted = (10.0_f32.powf(dbm_value / 10.0_f32)) / 1000.0_f32;

        Watt::new(converted)
    }
}
impl Add for Watt {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Watt::new(self.power + other.power)
    }
}

impl Sub for Watt {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Watt::new(self.power - other.power)
    }
}

impl Mul<f32> for Watt {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Watt::new(self.power * rhs)
    }
}

impl Mul<Watt> for f32 {
    type Output = Watt;

    fn mul(self, rhs: Watt) -> Self::Output {
        Watt::new(self * rhs.power)
    }
}

impl Div<f32> for Watt {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Watt::new(self.power / rhs)
    }
}

impl FromStr for Watt {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim().parse::<f32>() {
            Ok(num) => Ok(Watt::new(num)),
            Err(_) => Err(format!("Invalid power format: '{}'", s)),
        }
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ------------------------------dBm------------------------------ //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "stores", derive(Patch))]
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
impl From<Watt> for Dbm {
    fn from(watt_value: Watt) -> Dbm {
        let watt_value: f32 = watt_value.into();

        let converted = 10.0 * (watt_value * 1000.0).log10();

        Dbm::new(converted)
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// ------------------------------ADC------------------------------ //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "stores", derive(Patch))]
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
#[cfg_attr(feature = "stores", derive(Patch))]
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
#[cfg_attr(feature = "stores", derive(Patch))]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "stores", derive(Patch))]
/// Units of degC.
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "stores", derive(Patch))]
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
// ----------------------------Phase------------------------------ //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "stores", derive(Patch))]
pub struct Phase {
    /// Values are in degrees.
    pub phase: i16,
}
impl Phase {
    /// Creates a new phase operator in units of degrees.
    pub fn new(phase: i16) -> Self {
        Self { phase }
    }
}
impl Into<i16> for Phase {
    fn into(self) -> i16 {
        self.phase
    }
}
impl Display for Phase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.phase)
    }
}
impl Add for Phase {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Phase::new(self.phase.wrapping_add(other.phase))
    }
}

impl Sub for Phase {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Phase::new(self.phase.wrapping_sub(other.phase))
    }
}

impl FromStr for Phase {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim().parse::<i16>() {
            Ok(num) => Ok(Phase::new(num)),
            Err(_) => Err(format!("Invalid phase format: '{}'", s)),
        }
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// -------------------------Attenuation--------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "stores", derive(Patch))]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "stores", derive(Patch))]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "stores", derive(Patch))]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "stores", derive(Patch))]
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
#[cfg_attr(feature = "stores", derive(Patch))]
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

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Baud Rate---------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "stores", derive(Patch))]
pub struct BaudRate {
    pub baud_rate: u32,
}
impl BaudRate {
    pub fn new(baud_rate: u32) -> Self {
        Self { baud_rate }
    }
}
impl Default for BaudRate {
    fn default() -> Self {
        return Self { baud_rate: 115_200 };
    }
}
impl Into<u32> for BaudRate {
    fn into(self) -> u32 {
        self.baud_rate
    }
}
impl Display for BaudRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.baud_rate)
    }
}
