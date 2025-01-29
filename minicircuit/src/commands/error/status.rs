use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

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

        let hex_status_code = match parts[3].trim().parse::<u32>() {
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
    /// A hexadecimal value of the error on the ISC board.
    pub error_code: u32,
    /// The status of the ISC board.
    pub status: String,
    /// A description of the status.
    pub description: String,
}

impl Status {
    pub fn new(error_code: u32, status: String, description: String) -> Self {
        Self {
            error_code,
            status,
            description,
        }
    }

    /// Converts a hexadecimal value into a vector of `Status` codes by decoding the bitfield.
    pub fn from_hex_code(hex_code: u32) -> Vec<Self> {
        let mut statuses = Vec::new();

        // Check each bit in the bitfield and map it to a Status using match logic
        for bit_position in 0..24 {
            let bit_value = 1 << bit_position; // Compute the current bit value

            if hex_code & bit_value != 0 {
                match bit_value {
                    0x1 => statuses.push(Status::new(
                        1,
                        "Unspecified Error.".to_string(),
                        "RF output OFF (blocking); Reset controller.".to_string(),
                    )),
                    0x2 => statuses.push(Status::new(
                        2,
                        "High temperature in the PA.".to_string(),
                        "Unit throttles output power (with autogain enabled).".to_string(),
                    )),
                    0x4 => statuses.push(Status::new(
                        4,
                        "Shutdown Temperature in the PA.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x8 => statuses.push(Status::new(
                        8,
                        "High reflection.".to_string(),
                        "Warning only; no action.".to_string(),
                    )),
                    0x10 => statuses.push(Status::new(
                        10,
                        "Shutdown reflection.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x20 => statuses.push(Status::new(
                        20,
                        "Reset detected.".to_string(),
                        "Warning only; no action.".to_string(),
                    )),
                    0x40 => statuses.push(Status::new(
                        40,
                        "Temperature read-out error.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x80 => statuses.push(Status::new(
                        80,
                        "Power measurement failure.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x100 => statuses.push(Status::new(
                        100,
                        "RF output enable failure.".to_string(),
                        "Indication message.".to_string(),
                    )),
                    0x200 => statuses.push(Status::new(
                        200,
                        "Multiplexer failure (I2C).".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x400 => statuses.push(Status::new(
                        400,
                        "External shutdown triggered.".to_string(),
                        "RF output OFF (non-blocking).".to_string(),
                    )),
                    0x800 => statuses.push(Status::new(
                        800,
                        "Reserved.".to_string(),
                        "Error code reserved.".to_string(),
                    )),
                    0x1000 => statuses.push(Status::new(
                        1000,
                        "I2C communication problem has occurred.".to_string(),
                        "RF output OFF (blocking)in case of critical measurements.".to_string(),
                    )),
                    0x2000 => statuses.push(Status::new(
                        2000,
                        "SPI communication problem has occurred.".to_string(),
                        "RF output OFF (blocking)in case of critical measurements.".to_string(),
                    )),
                    0x4000 => statuses.push(Status::new(
                        4000,
                        "IQ Conversion Error.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x8000 => statuses.push(Status::new(
                        8000,
                        "SOA Measurement Error.".to_string(),
                        " RF output OFF (blocking).".to_string(),
                    )),
                    0x10000 => statuses.push(Status::new(
                        10000,
                        "External Communication Watchdog Timeout.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x20000 => statuses.push(Status::new(
                        20000,
                        "Calibration missing.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x40000 => statuses.push(Status::new(
                        40000,
                        "Reserved.".to_string(),
                        "Error code reserved.".to_string(),
                    )),
                    0x80000 => statuses.push(Status::new(
                        80000,
                        "SOA high dissipation.".to_string(),
                        "Warning only; no action.".to_string(),
                    )),
                    0x100000 => statuses.push(Status::new(
                        100000,
                        "SOA shutdown dissipation.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x200000 => statuses.push(Status::new(
                        200000,
                        "EEPROM content incompatible with firmware version.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x400000 => statuses.push(Status::new(
                        400000,
                        "Internal PA Error.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x800000 => statuses.push(Status::new(
                        800000,
                        "PA Reset Failure.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    0x1000000 => statuses.push(Status::new(
                        1000000,
                        "High Current.".to_string(),
                        "RF output OFF (blocking).".to_string(),
                    )),
                    _ => statuses.push(Status::new(
                        0,
                        "No error.".to_string(),
                        "Status is nominal".to_string(),
                    )),
                }
            }
        }

        statuses
    }
}
