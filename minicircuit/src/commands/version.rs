use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VersionResponse {
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

impl TryFrom<String> for VersionResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 7 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        let manufacturer_id = parts[0].to_string();
        let major_version = parts[1].to_string();
        let minor_version = parts[2].to_string();
        let build = parts[3].to_string();
        let hotfix = Some(parts[4].to_string());
        let date_stamp = parts[5].to_string();
        let time_stamp = parts[6].to_string();

        todo!();

        Ok(VersionResponse {
            manufacturer_id,
            major_version,
            minor_version,
            build,
            hotfix,
            date_stamp,
            time_stamp,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetVersion {
    channel: u8,
}

impl Into<String> for GetVersion {
    fn into(self) -> String {
        format!("$VER,{}", self.channel)
    }
}

impl GetVersion {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetVersion {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
