use serialport::{Error, ErrorKind, SerialPort};

/// A function to send commands to the serial port and receive it's response.
pub fn write_read(port: &mut dyn SerialPort, tx: String) -> Result<String, Error> {
    // Format the command to the ISC's standards.
    let command = format!("{}\r\n", tx);

    if let Err(e) = port.write_all(command.as_bytes()) {
        return Err(Error::new(
            ErrorKind::Io(e.kind()),
            format!("Failed to write to the port: {:?}", e),
        ));
    }

    let mut buffer = String::new();
    let mut serial_buf: Vec<u8> = vec![0; 1000];

    while !buffer.contains("\n") && !buffer.contains("\r") {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                buffer.push_str(&String::from_utf8_lossy(&serial_buf[..t]));
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                return Err(Error::new(
                    ErrorKind::Io(std::io::ErrorKind::TimedOut),
                    "System timedout while waiting for response from the controller.",
                ));
            }
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Io(e.kind()),
                    format!("Failed to read from the port: {:?}", e),
                ));
            }
        }
    }

    Ok(buffer.trim().to_string())
}
