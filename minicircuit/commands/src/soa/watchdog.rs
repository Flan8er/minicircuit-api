use serde::{Deserialize, Serialize};

use crate::data_types::{errors::MWError, types::Channel};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSOAWatchdogConfigResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetSOAWatchdogConfigResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetSOAWatchdogConfigResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Turns the software watchdog ON or OFF
///
/// The software watchdog is a function of the firmware which ensures that the various software components of the
/// firmware keep working as intended.
///
/// The following software components are guarded by the watchdog:
///
/// - Serial command interpreter / UART bus
///
/// - I^2C bus
///
/// - PWM trigger
///
/// - Safe Operating Area
///
/// - Auto-gain
///
/// - DLL algorithm
///
/// - USB bus
///
/// - Debug thread
///
/// The software watchdog sends requests to each of the components to confirm whether they
/// are still running. If the component fails to respond too many times in a row,
/// the watchdog triggers and the ISC board is automatically reset.
pub struct SetSOAWatchdogConfig {
    /// Channel identification number.
    pub channel: Channel,
    /// Enable state of the software watchdog
    ///
    /// True = ON (default)
    ///
    /// False = OFF
    pub enabled: bool,
}

impl Into<String> for SetSOAWatchdogConfig {
    fn into(self) -> String {
        format!("$SWES,{},{}", self.channel, self.enabled)
    }
}

impl SetSOAWatchdogConfig {
    /// Returns a handler to call the command using the given inputs.
    pub fn new(channel: Channel, enabled: bool) -> Self {
        Self { channel, enabled }
    }
}

impl Default for SetSOAWatchdogConfig {
    /// Returns the default handler to call the command.
    /// By default, watchdog is enabled.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            enabled: true,
        }
    }
}
