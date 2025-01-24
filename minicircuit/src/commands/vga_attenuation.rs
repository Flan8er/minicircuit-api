use serde::{Deserialize, Serialize};

use crate::errors::MWError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetVGAAttenuationDBResponse {
    /// The uptime in seconds.
    pub attenuation: f32,
}

impl TryFrom<String> for GetVGAAttenuationDBResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        // Parse a response string into the `IdentityResponse` struct
        let parts: Vec<&str> = response.split(',').collect();
        if parts.len() != 3 {
            // could be a error code here so instead check to see if there's an error code and pass it into the new:: function
            return Err(MWError::FailedParseResponse);
        }

        todo!();

        let attenuation = match parts[0].parse() {
            Ok(attenuation) => attenuation,
            Err(_) => return Err(MWError::FailedParseResponse),
        };

        Ok(GetVGAAttenuationDBResponse { attenuation })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// This command returns the configured attenuation value of the VGA which regulates the ISC board's poweroutput.
/// The higher the value, the lower the power output.
pub struct GetVGAAttenuationDB {
    channel: u8,
}

impl Into<String> for GetVGAAttenuationDB {
    fn into(self) -> String {
        format!("$GCG,{}", self.channel)
    }
}

impl GetVGAAttenuationDB {
    pub fn new(self, channel: u8) -> Self {
        Self { channel }
    }
}

impl Default for GetVGAAttenuationDB {
    fn default() -> Self {
        Self { channel: 1 }
    }
}
