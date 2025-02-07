use serde::{Deserialize, Serialize};

use super::data_types::types::BaudRate;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// The properties that are used for automatically detecting and
/// connecting to the signal generator and rules for the connection.
pub struct TargetProperties {
    pub port: Option<String>,
    /// The target vendor ID for connecting.
    ///
    /// When automatically connecting to signal generators
    /// this value is used to verify connecting to the proper generator.
    pub vendor_id: VendorId,
    /// The target product ID for connecting.
    ///
    /// When automatically connecting to signal generators
    /// this value is used to verify connecting to the proper generator.
    pub product_id: ProductId,
    /// The baud rate of the serial connection.
    pub baud_rate: BaudRate,
    /// The data bit structure of the serial connection.
    pub data_bits: serialport::DataBits,
    /// The parity bit structure for the serial connection.
    pub parity: serialport::Parity,
    /// The flow control mode for the serial connection.
    pub flow_control: serialport::FlowControl,
    /// The stop bit structure for the serial connection.
    pub stop_bits: serialport::StopBits,
    /// The timeout limit for attempting connection with the signal genertor.
    ///
    /// If the timeout limit is reached, the connection will fail.
    pub connection_timeout: std::time::Duration,
}

impl TargetProperties {
    pub fn new(
        port: Option<String>,
        vendor_id: VendorId,
        product_id: ProductId,
        baud_rate: BaudRate,
        data_bits: serialport::DataBits,
        parity: serialport::Parity,
        flow_control: serialport::FlowControl,
        stop_bits: serialport::StopBits,
        connection_timeout: std::time::Duration,
    ) -> Self {
        return Self {
            port,
            vendor_id,
            product_id,
            baud_rate,
            data_bits,
            parity,
            flow_control,
            stop_bits,
            connection_timeout,
        };
    }
}

impl Default for TargetProperties {
    fn default() -> Self {
        return Self {
            port: Some(String::from("/dev/tty.usbserial-FTBXKGR7")),
            vendor_id: VendorId::default(),
            product_id: ProductId::default(),
            baud_rate: BaudRate::default(),
            data_bits: serialport::DataBits::Eight,
            parity: serialport::Parity::None,
            flow_control: serialport::FlowControl::None,
            stop_bits: serialport::StopBits::One,
            connection_timeout: std::time::Duration::from_secs(1),
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VendorId {
    pub vendor_id: u16,
}
impl VendorId {
    pub fn new(vendor_id: u16) -> Self {
        Self { vendor_id }
    }
}
impl Into<u16> for VendorId {
    fn into(self) -> u16 {
        self.vendor_id
    }
}
impl Default for VendorId {
    fn default() -> Self {
        // Possible other values:
        // pub const TARGET_VENDOR_ID: u16 = 8137;
        Self { vendor_id: 1027 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProductId {
    pub product_id: u16,
}
impl ProductId {
    pub fn new(product_id: u16) -> Self {
        Self { product_id }
    }
}
impl Into<u16> for ProductId {
    fn into(self) -> u16 {
        self.product_id
    }
}
impl Default for ProductId {
    fn default() -> Self {
        // Possible other values:
        // pub const TARGET_PRODUCT_ID: u16 = 131;
        Self { product_id: 24577 }
    }
}
