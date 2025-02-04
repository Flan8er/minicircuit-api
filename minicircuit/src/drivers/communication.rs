use serialport::{Error, ErrorKind, SerialPort};
use std::time::{Duration, Instant};

use crate::commands::command::Command;

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

    if let Err(e) = port.flush() {
        return Err(Error::new(
            ErrorKind::Io(e.kind()),
            format!("Failed to flush the port: {:?}", e),
        ));
    }

    let mut buffer = String::new();
    let mut temp_buffer = [0; 256];
    let timeout = Duration::from_millis(500);
    let start_time = Instant::now();

    while buffer != String::new() {
        if start_time.elapsed() >= timeout {
            return Err(Error::new(
                ErrorKind::Io(std::io::ErrorKind::TimedOut),
                "Timeout while waiting for response.",
            ));
        }

        match port.read(&mut temp_buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    buffer.push_str(&String::from_utf8_lossy(&temp_buffer[..bytes_read]));
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Io(e.kind()),
                    format!("Failed to read from the port: {:?}", e),
                ))
            }
        }
    }

    Ok(buffer)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadWriteError {
    /// The command the error is associated with.
    pub command: Command,
    pub error_kind: ErrorKind,
    /// A description of the error.
    pub description: String,
}

impl ReadWriteError {
    pub fn new(command: Command, error_kind: ErrorKind, description: String) -> Self {
        Self {
            command,
            error_kind,
            description,
        }
    }
}
