use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearErrorsResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for ClearErrorsResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if !response.contains("OK") {
            // Parsing the error depends on the location of the substring "ERR"
            // which in this command occurs twice: "$ERRC,1,OK"
            //
            // trimmed_response is just removing the first occurance to be able
            // to parse any errors from executing this command.
            let trimmed_response: String = response.replace("$ERRC", "");

            let response_error: Self::Error = trimmed_response.into();
            return Err(response_error);
        }

        Ok(ClearErrorsResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Clears the error state of the ISC board and resets the protective systems
/// that impede the board while an error is present.
pub struct ClearErrors {
    /// Desired channel identification number.
    pub channel: Channel,
}

impl Into<String> for ClearErrors {
    fn into(self) -> String {
        format!("$ERRC,{}", self.channel)
    }
}

impl ClearErrors {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for ClearErrors {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
