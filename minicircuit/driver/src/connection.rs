use serialport::{available_ports, Error, SerialPort, SerialPortInfo};

use super::properties::{ProductId, TargetProperties, VendorId};

/// Used for connecting directly to the supplied port in the target properties.
///
/// Use this method if the port location is guaranteed.
///
/// Opens the port using the remaining target properties.
pub fn open_port(target_properties: TargetProperties) -> Option<Box<dyn SerialPort>> {
    let Some(desired_port) = target_properties.port else {
        eprintln!("Port required for direct connection.");
        return None;
    };

    let desired_baud_rate: u32 = target_properties.baud_rate.into();

    match serialport::new(&desired_port, desired_baud_rate)
        .data_bits(target_properties.data_bits)
        .parity(target_properties.parity)
        .flow_control(target_properties.flow_control)
        .stop_bits(target_properties.stop_bits)
        .timeout(target_properties.connection_timeout)
        .open()
    {
        Ok(port) => Some(port),
        Err(e) => {
            eprintln!("Failed to open port \"{}\". Error: {}", desired_port, e);
            None
        }
    }
}

/// This function auto-detects and connects to the port that meets the required product and manufacturer IDs  passed in the inputs.
/// Does not use the supplied port as it aims to be as flexible as possible in the connection.
///
/// Use this method if the port location isn't guaranteed.
///
/// Multiple ports could meet the same requirements; in this case the first port is chosen.
pub fn connect_to_signal_generator(
    target_properties: TargetProperties,
) -> Option<Box<dyn SerialPort>> {
    // Get a list of ports that match the vendor and product ids with those of the target properties.
    let Ok(signal_generators) =
        autodetect_sg_port(target_properties.vendor_id, target_properties.product_id)
    else {
        return None;
    };

    // Verify a port was detected.
    if signal_generators.is_empty() {
        return None;
    }

    // Connect to the first port that matches the requirements.
    let first_signal_generator = &signal_generators[0];

    // Open a serial connection with the detected port at the requested settings.
    match serialport::new(
        &first_signal_generator.port_name,
        target_properties.baud_rate.into(),
    )
    .data_bits(target_properties.data_bits)
    .parity(target_properties.parity)
    .flow_control(target_properties.flow_control)
    .stop_bits(target_properties.stop_bits)
    .timeout(target_properties.connection_timeout)
    .open()
    {
        Ok(port) => Some(port),
        Err(e) => {
            eprintln!(
                "Failed to open port \"{}\". Error: {}",
                first_signal_generator.port_name, e
            );
            None
        }
    }
}

pub fn autodetect_sg_port(
    vendor_id: VendorId,
    product_id: ProductId,
) -> Result<Vec<SerialPortInfo>, Error> {
    // Get a list of available coms ports.
    let available_ports = match available_ports() {
        Ok(ports) => ports,
        Err(e) => {
            return Err(Error::new(
                serialport::ErrorKind::Unknown,
                format!("Failed to list serial ports: {:?}", e),
            ));
        }
    };

    // Return the ports that match the requested vendor and product ids.
    Ok(available_ports
        .into_iter()
        .filter(|port| {
            if let serialport::SerialPortType::UsbPort(usb_info) = &port.port_type {
                let vendor_id: u16 = vendor_id.clone().into();
                let product_id: u16 = product_id.clone().into();

                let Some(product) = usb_info.clone().product else {
                    return false;
                };

                // The filter requirement for returning the port is that the product and vendor ids match the requested ids.
                usb_info.vid == vendor_id
                    && usb_info.pid == product_id
                    && port.port_name.contains("tty")
                    && !product.contains("UART")
            } else {
                false
            }
        })
        .collect())
}

pub fn print_available_ports() {
    // Get a list of available coms ports.
    let available_ports = match available_ports() {
        Ok(ports) => ports,
        Err(e) => return,
    };

    println!("All available ports are {:#?}", available_ports)
}
