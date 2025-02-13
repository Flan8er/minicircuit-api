use serde::{Deserialize, Serialize};

use crate::data_types::{errors::MWError, types::Channel};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// The current version of the firmware.
pub struct GetVersionResponse {
    // Firmware developer identifier.
    pub manufacturer_id: String,
    /// The version's major revision number.
    pub major_version: String,
    /// The version's minor revision number.
    pub minor_version: String,
    /// The verion's build number.
    pub build: String,
    /// Optional version hotfix number.
    pub hotfix: Option<String>,
    /// The date on which the firmware was compiled.
    pub date_stamp: String,
    /// The time at which the firmware was compiled.
    pub time_stamp: String,
}

impl TryFrom<String> for GetVersionResponse {
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
        let parsed_response = match parts.len() {
            8 => parse_without_hotfix(parts),
            9 => parse_with_hotfix(parts),
            _ => {
                return Err(Self::Error::FailedParseResponse);
            }
        };

        Ok(parsed_response)
    }
}

fn parse_with_hotfix(parts: Vec<&str>) -> GetVersionResponse {
    let manufacturer_id = parts[2].trim().to_string();
    let major_version = parts[3].trim().to_string();
    let minor_version = parts[4].trim().to_string();
    let build = parts[5].trim().to_string();
    let hotfix = parts[6].trim().to_string();
    let date_stamp = parts[7].trim().to_string();
    let time_stamp = parts[8].trim().to_string();

    GetVersionResponse {
        manufacturer_id,
        major_version,
        minor_version,
        build,
        hotfix: Some(hotfix),
        date_stamp,
        time_stamp,
    }
}
fn parse_without_hotfix(parts: Vec<&str>) -> GetVersionResponse {
    let manufacturer_id = parts[2].trim().to_string();
    let major_version = parts[3].trim().to_string();
    let minor_version = parts[4].trim().to_string();
    let build = parts[5].trim().to_string();
    let date_stamp = parts[6].trim().to_string();
    let time_stamp = parts[7].trim().to_string();

    GetVersionResponse {
        manufacturer_id,
        major_version,
        minor_version,
        build,
        hotfix: None,
        date_stamp,
        time_stamp,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Returns the current version of the firmware.
pub struct GetVersion {
    /// Desired channel identification number.
    pub channel: Channel,
}

impl Into<String> for GetVersion {
    fn into(self) -> String {
        format!("$VER,{}", self.channel)
    }
}

impl GetVersion {
    /// Returns a handler to call the command.
    /// Use ::default() if channel specifier isn't unique.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Default for GetVersion {
    /// Returns the default handler to call the command.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
        }
    }
}
