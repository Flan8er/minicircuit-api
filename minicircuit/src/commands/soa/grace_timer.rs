use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOAGraceTimerResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOAGraceTimerResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOAGraceTimerResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Configures the grace period for the SOA's protection systems.
///
/// There may be situations where it is desirable to permit a grace period before SOA acts
/// and potentially shuts down everything. The SOA grace timer may be used to allow temporary violations
/// of the reflection, dissipation, and temperature limits for a configurable period. Only a continuous,
/// uninterrupted violation longer than the grace timeout will trigger a reaction from the SOA.
pub struct SetSOAGraceTimer {
    /// Channel identification number.
    pub channel: Channel,
    /// The period in milliseconds that SOA should tolerate violations before taking action.
    pub grace_period: u16,
}

impl Into<String> for SetSOAGraceTimer {
    fn into(self) -> String {
        format!("$SOAGS,{},{}", self.channel, self.grace_period)
    }
}

impl SetSOAGraceTimer {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(self, channel: Channel, grace_period: u16) -> Self {
        Self {
            channel,
            grace_period,
        }
    }
}

impl Default for SetSOAGraceTimer {
    /// Returns the default handler to call the command.
    /// By default, the grace period is set to 500ms.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            grace_period: 500,
        }
    }
}
