use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// ISC-(frequency_low)(frequency_high)-(power)+
///
/// (frequency_low) - Lower frequency limit (only first 2 digits).
///
/// (frequency_high) - Upper frequency limit (only first 2 digits).
///
/// (power) - Maximum RF output power of the signal generator board in dBm.
///
/// Ex: ISC-2425-25+
pub struct GetIdentityResponse {
    /// Name of the manufacturer.
    pub manufacturer: String,
    /// The type of ISC board.
    pub isc_board: String,
    /// Unique serial number of the board.
    pub serial_number: String,
}

impl TryFrom<String> for GetIdentityResponse {
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

        let manufacturer = parts[2].trim().to_string();
        let isc_board = parts[3].trim().to_string();
        let serial_number = parts[4].trim().to_string();

        Ok(GetIdentityResponse {
            manufacturer,
            isc_board,
            serial_number,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the identity of the ISC board.
pub struct GetIdentity {
    /// Desired channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetIdentity {
    fn into(self) -> String {
        format!("$IDN,{}", self.channel)
    }
}

impl GetIdentity {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetIdentity {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
