use regex::Regex;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use log::{error, info};
use minicircuit_simulate::simulator::MiniCircuitSimulator;

// Default COM port constants
const DEFAULT_WINDOWS_CLIENT_PORT: &str = "COM5";
const DEFAULT_WINDOWS_DEVICE_PORT: &str = "COM6";

fn main() {
    // Initialize logger
    env_logger::init();

    println!("Starting MiniCircuit Simulator");
    println!("This program simulates a MiniCircuit device on your computer");
    println!("Other applications can connect to it as if it were a real device");

    // Set up virtual serial ports using socat
    let (client_port, device_port) = if cfg!(unix) {
        match setup_socat_ports() {
            Ok(ports) => ports,
            Err(e) => {
                error!("Failed to set up socat ports: {}", e);
                println!("Error: {}", e);
                println!("Please make sure 'socat' is installed and accessible in your PATH.");
                println!("On macOS, run: brew install socat");
                println!("On Debian/Ubuntu, run: sudo apt-get install socat");
                return;
            }
        }
    } else {
        // For Windows, use default hardcoded ports
        (
            DEFAULT_WINDOWS_CLIENT_PORT.to_string(),
            DEFAULT_WINDOWS_DEVICE_PORT.to_string(),
        )
    };

    if cfg!(unix) {
        // Add a delay to allow socat to initialize the ports
        thread::sleep(Duration::from_secs(1));
    }

    println!("Virtual serial ports created:");
    println!("  - Client port: {}", client_port);
    println!("  - Device port: {}", device_port);
    println!("\nConnect your application to: {}", client_port);

    // Create the simulator instance
    let simulator = Arc::new(Mutex::new(MiniCircuitSimulator::new()));

    // Start the simulator on the device port
    let simulator_clone = simulator.clone();
    let simulator_thread = thread::spawn(move || {
        run_simulator(simulator_clone, &device_port);
    });

    // Print command log periodically
    let _log_thread = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));

        let simulator_guard = simulator.lock().unwrap();
        let log = simulator_guard.get_command_log();
        if !log.is_empty() {
            println!("\nCommand log (last 5 commands):");
            for (i, cmd) in log.iter().rev().take(5).enumerate() {
                println!("  {}: {}", log.len() - i, cmd);
            }
        }
    });

    println!("\nSimulator is running. Press Ctrl+C to exit.");

    // Wait for the simulator thread to finish (it won't unless there's an error)
    simulator_thread.join().unwrap();
}

fn setup_socat_ports() -> Result<(String, String), Box<dyn std::error::Error>> {
    println!("Setting up virtual serial ports with socat...");

    let mut child = Command::new("socat")
        .args(&["-d", "-d", "pty,raw,echo=0", "pty,raw,echo=0"])
        .stderr(Stdio::piped())
        .spawn()?;

    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
    let reader = BufReader::new(stderr);

    let re = Regex::new(r"PTY is (/dev/ttys\d+)")?;
    let mut ports = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            ports.push(caps.get(1).unwrap().as_str().to_string());
            if ports.len() == 2 {
                break;
            }
        }
    }

    std::mem::forget(child);

    if ports.len() == 2 {
        Ok((ports[0].clone(), ports[1].clone()))
    } else {
        Err("Could not find both PTY ports from socat output. Make sure socat is installed.".into())
    }
}

fn run_simulator(simulator: Arc<Mutex<MiniCircuitSimulator>>, port_name: &str) {
    info!("Starting simulator on port: {}", port_name);
    println!("Attempting to open port: {}", port_name);

    // List available ports for diagnostic purposes
    println!("Available ports:");
    match serialport::available_ports() {
        Ok(ports) => {
            for port in ports {
                println!("  - {}", port.port_name);
            }
        }
        Err(e) => println!("Error listing ports: {}", e),
    }

    // Open the serial port with a shorter timeout
    let mut port = match serialport::new(port_name, 0)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(port) => {
            println!("Successfully opened port {}", port_name);
            port
        }
        Err(e) => {
            error!("Failed to open port {}: {}", port_name, e);
            println!("Failed to open port {}: {}", port_name, e);
            println!("The simulator will not be able to receive commands.");
            println!("Make sure the port exists and is not in use by another application.");
            println!("Try running the simulator as administrator if you're on Windows.");

            // Wait for user to see the error
            println!("Press Enter to exit...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            return;
        }
    };

    // Buffer for reading commands
    let mut buffer = [0u8; 1024];
    let mut command_buffer = Vec::new();

    println!("Simulator ready to receive commands on port {}", port_name);

    loop {
        // Read data from the port
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    // Convert bytes to string
                    let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Received data: {:?}", data);

                    // Process each character
                    for c in data.chars() {
                        if c == '\r' || c == '\n' {
                            // End of command
                            if !command_buffer.is_empty() {
                                // Process the command
                                let command = String::from_iter(command_buffer.iter());
                                println!("Received command: {}", command);

                                // Get the response from the simulator
                                let response = {
                                    let mut sim = simulator.lock().unwrap();
                                    sim.process_command(&command)
                                };

                                // Special handling for identity command
                                if command.contains("$IDN") {
                                    println!("Identity command detected: {}", command);
                                    println!("Identity response: {}", response);
                                }

                                // Send the response back immediately
                                let response_with_newline = format!("{}\r\n", response);
                                println!("Sending response: {}", response);
                                if let Err(e) = port.write_all(response_with_newline.as_bytes()) {
                                    error!("Failed to write response: {}", e);
                                }
                                port.flush()
                                    .unwrap_or_else(|e| error!("Failed to flush port: {}", e));

                                // Clear the command buffer for the next command
                                command_buffer.clear();
                            }
                        } else {
                            // Add character to command buffer
                            command_buffer.push(c);
                        }
                    }
                }
            }
            Err(e) => {
                // Ignore timeout errors as they're expected when no data is available
                if e.kind() != std::io::ErrorKind::TimedOut {
                    error!("Error reading from port: {}", e);
                }
            }
        }

        // Print the command log periodically
        if let Ok(sim) = simulator.try_lock() {
            let log = sim.get_command_log();
            if !log.is_empty() && log.len() % 5 == 0 {
                println!("Command log (last {} commands):", log.len().min(5));
                for cmd in log.iter().rev().take(5) {
                    println!("  {}", cmd);
                }
            }
        }

        // Small delay to prevent CPU spinning
        std::thread::sleep(Duration::from_millis(10));
    }
}
