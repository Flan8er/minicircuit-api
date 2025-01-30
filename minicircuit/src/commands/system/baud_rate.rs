use serde::{Deserialize, Serialize};

use crate::drivers::data_types::types::Channel;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// THIS COMMAND DOES NOT REPLY.
///
/// Sets the baud rate used for communicating through UART.
/// Any value can be entered, but unsurprisingly, ongoing
/// communication will break the moment this value is changed.
///
/// Changing the baud rate affects communication speed. Lowering it
/// can cause noticable communication delays, while increasing it can
/// speed up communication and leave a larger CPU time-slice for
/// other tasks. However, setting the baud rate too high may cause
/// communication issues to arise, as the UART transceivers have limitations.
///
/// After changing the baud rate, the communication line needs to be reinitialized
/// on the user side with the updated baud values.
///
/// This setting does not affect communication through USB, only through UART.
pub struct SetUartBaudRate {
    /// Channel identification number.
    pub channel: Channel,
    /// Baud rate in symbols per second. For UART to work, the baud rate on the
    /// Tx and Rx side must be configured to the same value.
    pub baud_rate: u32,
}

impl Into<String> for SetUartBaudRate {
    fn into(self) -> String {
        format!("$UARTS,{},{}", self.channel, self.baud_rate)
    }
}

impl SetUartBaudRate {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(self, channel: Channel, baud_rate: u32) -> Self {
        Self { channel, baud_rate }
    }
}

impl Default for SetUartBaudRate {
    /// Returns the default handler to call the command.
    ///
    /// By default, baud rate is set to 115_200.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            baud_rate: 115_200,
        }
    }
}
