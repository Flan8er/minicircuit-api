use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StatusResponse {
    /// Reserved value which will always return 0.
    reserved: u8,
    /// Hexadecimal value representing various errors which have occurred on the ISCboard.
    pub status_code: Vec<Status>,
}

impl TryFrom<String> for StatusResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 2 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let status_code = match parts[0].parse() {
            Ok(code) => code,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        let decoded_statuses = Status::from_hex_code(status_code);

        Ok(StatusResponse {
            reserved: 0,
            status_code: decoded_statuses,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetStatus {
    channel: u8,
}

impl Into<String> for GetStatus {
    fn into(self) -> String {
        format!("$ST,{}", self.channel)
    }
}

impl GetStatus {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetStatus {
    fn default() -> Self {
        Self { channel: 1 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Status {
    /// A hexadecimal value of the error on the ISCboard
    pub error_code: u32,
    pub status: String,
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
