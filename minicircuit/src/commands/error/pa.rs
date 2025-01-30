use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetPAErrorsResponse {
    /// Error code of the PA displayed in decimal. For reference,
    /// the codes of the ZHL-2425-250X+ are shown below:
    ///
    /// 2 bytes value where each bit represents an alarm cause as follows.
    /// (bits 8, 9, 12, 13, 14, and 15 are reserved).
    ///
    /// bit0: Reflected Power > Upper Limit
    ///
    /// bit1: Reflected Power < Lower Limit
    ///
    /// bit2: Forward Power > Upper Limit1
    ///
    /// bit3: Forward Power < Lower Limit1
    ///
    /// bit4: Current > Upper Limit
    ///
    /// bit5: Current < Lower Limit1
    ///
    /// bit6: V_Supply > Upper Limit
    ///
    /// bit7: V_Supply < Lower Limit
    ///
    /// bit10: Temperature > Upper Limit
    ///
    /// bit11: Temperature < Lower Limit
    ///
    /// Note: there is no protection limit set, so there should never be an internal
    /// alarm for these parameters.
    pub pa_error_code: u8,
}

impl TryFrom<String> for GetPAErrorsResponse {
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

        let pa_error_code: u8 = match parts[2].trim().parse::<u8>() {
            Ok(value) => value,
            Err(_) => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(GetPAErrorsResponse { pa_error_code })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Gets the status of the power amplifier (PA). If the status is 0, this indicates normal operation.
/// If the status is non-zero, one or more PA internal protection limits have been triggered.
/// Typically, this means that the PA will have already shut itself down in self-protection.
/// When the PA error code of a system in non-zero, it raises the `PAError` and triggers SOA `PAStatus`.
/// If an alarm signal is sent from the PA to the ISC, the `AlarmIn` error will also be raised. In multi-channel systems,
/// the returned error code status is a bitwise OR of the statuses of each channel.
pub struct GetPAErrors {
    /// Channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetPAErrors {
    fn into(self) -> String {
        format!("$PSG,{}", self.channel)
    }
}

impl GetPAErrors {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(self, channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetPAErrors {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
