use serde::{Deserialize, Serialize};

use crate::{drivers::data_types::types::Channel, errors::MWError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetCommunicationInterfaceResponse {
    /// The result of the command (Ok/Err).
    pub result: Result<(), MWError>,
}

impl TryFrom<String> for SetCommunicationInterfaceResponse {
    type Error = MWError;

    fn try_from(response: String) -> Result<Self, Self::Error> {
        if response.contains("ERR") {
            let response_error: Self::Error = response.into();
            return Err(response_error);
        }

        Ok(SetCommunicationInterfaceResponse { result: Ok(()) })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Sets the communication interface to UART (3.3V TTL) or USB. Only one communication
/// interface can be active at a time.
///
/// The default communication interface is USB. If the user switches to UART by sending a
/// `SetCommunicationInterface::new(Channel::default(), Interface::Uart)` command, the USB serial
/// port will no longer be active. COmmunication may only resume over UART during that session.
///
/// Rebooting will return the unit back to its default communication interface (USB).
pub struct SetCommunicationInterface {
    /// Channel identification number.
    pub channel: Channel,
    /// Serial communication interface.
    pub interface: Interface,
}

impl Into<String> for SetCommunicationInterface {
    fn into(self) -> String {
        let interface: u8 = self.interface.into();
        format!("$COMS,{},{}", self.channel, interface)
    }
}

impl SetCommunicationInterface {
    /// Returns a handler to call the command with specified inputs.
    pub fn new(self, channel: Channel, interface: Interface) -> Self {
        Self { channel, interface }
    }
}

impl Default for SetCommunicationInterface {
    /// Returns the default handler to call the command.
    ///
    /// By default, the interface is set to USB.
    fn default() -> Self {
        Self {
            channel: Channel::default(),
            interface: Interface::Usb,
        }
    }
}

// --------------------------------------------------------------- //
//                                                                 //
// --------------------------Interface---------------------------- //
//                                                                 //
// --------------------------------------------------------------- //
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Interface {
    Uart,
    Usb,
}
impl Into<u8> for Interface {
    fn into(self) -> u8 {
        return match self {
            Interface::Uart => 1,
            Interface::Usb => 2,
        };
    }
}
// impl From<u8> for Interface {
//     fn from(value: u8) -> Self {
//         return match value {
//             1 => Self::Uart,
//             _ => Self::Usb,
//         };
//     }
// }
