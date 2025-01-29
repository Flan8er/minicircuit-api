use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

macro_rules! define_status_codes {
    (
        $(
            $variant:ident => $bit:expr
        ),+ $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum StatusCode {
            $(
                $variant
            ),+
        }

        impl StatusCode {
            /// Convert an enum variant to its bit value.
            pub fn to_bit_value(&self) -> u64 {
                match self {
                    $(
                        StatusCode::$variant => $bit
                    ),+
                }
            }

            /// Convert a bit value to an enum variant (if it matches).
            pub fn from_bit_value(bit_value: u64) -> Option<StatusCode> {
                match bit_value {
                    $(
                        $bit => Some(StatusCode::$variant),
                    )+
                    _ => None,
                }
            }
        }
    }
}

define_status_codes! {
    SystemOk => 0x000000000,
    UnspecifiedError => 0x000000001,
    HighPATemperature => 0x000000002,
    ShutdownPATemperature => 0x000000004,
    HighReflectedPower => 0x000000008,
    ShutdownReflectedPower => 0x000000010,
    ResetDetected => 0x000000020,
    TemperatureReadoutError => 0x000000040,
    PowerMeasurementFailure => 0x000000080,
    RFEnableFailure => 0x000000100,
    MultiplexerFailure => 0x000000200,
    ExternalShutdownTriggered => 0x000000400,
    OutOfMemory => 0x000000800,
    I2CCommunicationError => 0x000001000,
    SPICommunicaitonError => 0x000002000,
    SOAMeasurementError => 0x000008000,
    ExternalWatchdogTimeout => 0x000010000,
    CalibrationMissing => 0x000020000,
    ExternalProtectionTriggered => 0x000040000,
    SOAHighDissipation => 0x000080000,
    SOAShutdownDissipation => 0x000100000,
    CalibrationEEPROMOutdated => 0x000200000,
    PAError => 0x000400000,
    PAResetFailure => 0x000800000,
    PAHighCurrent => 0x001000000,
    AlarmIn => 0x004000000,
    SOAHighCurrent => 0x010000000,
    SOAShutdownCurrent => 0x020000000,
    SOAHighForwardPower => 0x040000000,
    SOAShutdownForwardPower => 0x080000000,
    SOAShutdownMinimumVoltage => 0x100000000,
    SOALowVoltage => 0x200000000,
    SOAHighVoltage => 0x400000000,
    SOAShutdownMaximumVoltage => 0x800000000,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// List of status codes stored on the ISC board.
pub struct StatusResponse {
    pub status_codes: Vec<Status>,
}

impl TryFrom<String> for StatusResponse {
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

        let hex_status_code = match parts[3].trim().parse::<u64>() {
            Ok(value) => value,
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        let status_codes: Vec<Status> = Status::from_hex_code(hex_status_code);

        Ok(StatusResponse { status_codes })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Used to monitor the status of the ISC board.
///
/// ISC boards have a safety feature called the 'Safe Operating Area' (SOA).
/// If a fault occurs during operation, the SOA raises an error and takes action to protect the system.
/// This is indicated by a red LED on the board.
/// An error on the ISC board is accompanied by an informative error code which can be used to trace the problem.
///
/// To ensure the code can be viewed it stays in memory until manually cleared away.
/// For safety reasons some errors block the RF power output of the ISC board until cleared (ClearErrors).
pub struct GetStatus {
    /// Desired channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetStatus {
    fn into(self) -> String {
        format!("$ST,{}", self.channel)
    }
}

impl GetStatus {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetStatus {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Status {
    /// The status of the ISC board.
    pub status: String,
    /// A description of the status.
    pub description: String,
}

impl From<StatusCode> for Status {
    fn from(code: StatusCode) -> Status {
        match code {
            StatusCode::SystemOk => {
                return Self {
                    status: "No errors or warning".to_string(),
                    description: "Business as usual!".to_string(),
                }
            }
            StatusCode::UnspecifiedError => {
                return Self {
                    status: "Unspecified Error".to_string(),
                    description: "RF output OFF (blocking); Reset controller.".to_string(),
                }
            }
            StatusCode::HighPATemperature => return Self {
                status: "High PA Temperature".to_string(),
                description:
                    "With autogain enabled, unit throttles output power (see SOA for more detail)."
                        .to_string(),
            },
            StatusCode::ShutdownPATemperature => {
                return Self {
                    status: "Shutdown PA Temperature".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::HighReflectedPower => return Self {
                status: "High Reflected Power".to_string(),
                description:
                    "With autogain enabled, unit throttles output power (see SOA for more detail)."
                        .to_string(),
            },
            StatusCode::ShutdownReflectedPower => {
                return Self {
                    status: "Shutdown Reflected Power".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::ResetDetected => Self {
                status: "Reset Detected".to_string(),
                description: "Warning only - no action.".to_string(),
            },
            StatusCode::TemperatureReadoutError => {
                return Self {
                    status: "Temperature Read-out Error".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::PowerMeasurementFailure => {
                return Self {
                    status: "Power Measurement Failure".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::RFEnableFailure => {
                return Self {
                    status: "RF Enable Failure".to_string(),
                    description: "Warning only - no action.".to_string(),
                }
            }
            StatusCode::MultiplexerFailure => {
                return Self {
                    status: "Multiplexer Failure".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::ExternalShutdownTriggered => {
                return Self {
                    status: "External Shutdown Triggered".to_string(),
                    description: "RF output disabled (Non-Blocking).".to_string(),
                }
            }
            StatusCode::OutOfMemory => {
                return Self {
                    status: "Out of Memory".to_string(),
                    description: "Warning only - no action.".to_string(),
                }
            }
            StatusCode::I2CCommunicationError => {
                return Self {
                    status: "I2C Communication Error".to_string(),
                    description: "RF output disabled in case of critical measurement.".to_string(),
                }
            }
            StatusCode::SPICommunicaitonError => {
                return Self {
                    status: "SPI Communication Error".to_string(),
                    description: "RF output disabled in case of critical measurement.".to_string(),
                }
            }
            StatusCode::SOAMeasurementError => {
                return Self {
                    status: "SOA Measurement Error".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::ExternalWatchdogTimeout => {
                return Self {
                    status: "External Watchdog Timeout".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::CalibrationMissing => {
                return Self {
                    status: "Calibration Missing".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::ExternalProtectionTriggered => {
                return Self {
                    status: "External Protection Triggered".to_string(),
                    description: "Warning only - no action.".to_string(),
                }
            }
            StatusCode::SOAHighDissipation => {
                return Self {
                    status: "SOA High Dissipation".to_string(),
                    description: "Warning only - no action.".to_string(),
                }
            }
            StatusCode::SOAShutdownDissipation => {
                return Self {
                    status: "SOA Shutdown Dissipation".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::CalibrationEEPROMOutdated => {
                return Self {
                    status: "Calibration EEPROM Outdated".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::PAError => {
                return Self {
                    status: "PA Error".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::PAResetFailure => {
                return Self {
                    status: "PA Reset Failure".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::PAHighCurrent => {
                return Self {
                    status: "PA High Current".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::AlarmIn => {
                return Self {
                    status: "Alarm In".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::SOAHighCurrent => {
                return Self {
                    status: "SOA High Current".to_string(),
                    description: "Warning only - no action.".to_string(),
                }
            }
            StatusCode::SOAShutdownCurrent => {
                return Self {
                    status: "SOA Shutdown Current".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::SOAHighForwardPower => {
                return Self {
                    status: "SOA High Forward Power".to_string(),
                    description: "Warning only - no action".to_string(),
                }
            }
            StatusCode::SOAShutdownForwardPower => {
                return Self {
                    status: "SOA Shutdown Forward Power".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::SOAShutdownMinimumVoltage => {
                return Self {
                    status: "SOA Shutdown Minimum Voltage".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
            StatusCode::SOALowVoltage => {
                return Self {
                    status: "SOA Low Voltage".to_string(),
                    description: "Warning only - no action".to_string(),
                }
            }
            StatusCode::SOAHighVoltage => {
                return Self {
                    status: "SOA High Voltage".to_string(),
                    description: "Warning only - no action.".to_string(),
                }
            }
            StatusCode::SOAShutdownMaximumVoltage => {
                return Self {
                    status: "SOA Shutdown Maximum Voltage".to_string(),
                    description: "RF output disabled.".to_string(),
                }
            }
        }
    }
}

impl Status {
    pub fn new(status: &str, description: &str) -> Self {
        Self {
            status: status.to_string(),
            description: description.to_string(),
        }
    }

    pub fn from_hex_code(hex_code: u64) -> Vec<Self> {
        let mut statuses = Vec::new();

        // Use 0..32 or 0..64 depending on how high your bits can go
        for bit_position in 0..64 {
            let bit_mask = 1 << bit_position;

            // Check if that bit is actually set in `hex_code`
            if (hex_code & bit_mask) != 0 {
                // If this specific bit maps to a StatusCode, push it,
                // otherwise default to something like UnspecifiedError
                if let Some(status_code) = StatusCode::from_bit_value(bit_mask) {
                    statuses.push(Status::from(status_code));
                } else {
                    statuses.push(Status::from(StatusCode::UnspecifiedError));
                }
            }
        }

        statuses
    }
}
