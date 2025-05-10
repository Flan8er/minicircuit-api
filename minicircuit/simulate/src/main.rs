use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use log::{info, error};
use minicircuit_simulate::simulator::MiniCircuitSimulator;
use serialport::SerialPortType;

// Default COM port constants
const DEFAULT_WINDOWS_CLIENT_PORT: &str = "COM5";
const DEFAULT_WINDOWS_DEVICE_PORT: &str = "COM6";
const DEFAULT_UNIX_CLIENT_PORT: &str = "/dev/ttyS0";
const DEFAULT_UNIX_DEVICE_PORT: &str = "/dev/ttyS1";

fn main() {
    // Initialize logger
    env_logger::init();
    
    println!("Starting MiniCircuit Simulator");
    println!("This program simulates a MiniCircuit device on your computer");
    println!("Other applications can connect to it as if it were a real device");
    
    // Create the simulator instance
    let simulator = Arc::new(Mutex::new(MiniCircuitSimulator::new()));
    
    // Create a virtual COM port pair
    // On Windows, we'll use com0com (http://com0com.sourceforge.net/)
    // On Linux/Mac, we'll use socat or pty
    let port_pair = create_virtual_com_port_pair();
    
    println!("\nVirtual COM port created:");
    println!("  Client port: {}", port_pair.client_port);
    println!("  Device port: {}", port_pair.device_port);
    println!("\nConnect to the client port from your application using:");
    println!("  target_properties.port = Some(String::from(\"{}\"));", port_pair.client_port);
    
    // Start the simulator on the device port
    let simulator_clone = simulator.clone();
    let device_port = port_pair.device_port.clone();
    
    let simulator_thread = thread::spawn(move || {
        run_simulator(simulator_clone, &device_port);
    });
    
    // Print command log periodically
    let _log_thread = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            
            let simulator_guard = simulator.lock().unwrap();
            let log = simulator_guard.get_command_log();
            if !log.is_empty() {
                println!("\nCommand log (last 5 commands):");
                for (i, cmd) in log.iter().rev().take(5).enumerate() {
                    println!("  {}: {}", log.len() - i, cmd);
                }
            }
        }
    });
    
    println!("\nSimulator is running. Press Ctrl+C to exit.");
    
    // Wait for the simulator thread to finish (it won't unless there's an error)
    simulator_thread.join().unwrap();
}

struct PortPair {
    client_port: String,
    device_port: String,
}

fn create_virtual_com_port_pair() -> PortPair {
    // Detect the operating system
    let os = std::env::consts::OS;
    
    match os {
        "windows" => create_windows_com_port_pair(),
        "linux" | "macos" => create_unix_com_port_pair(),
        _ => {
            println!("Unsupported operating system: {}", os);
            println!("Using dummy port names. The simulator will not actually work.");
            PortPair {
                client_port: DEFAULT_WINDOWS_CLIENT_PORT.to_string(),
                device_port: DEFAULT_WINDOWS_DEVICE_PORT.to_string(),
            }
        }
    }
}

fn create_windows_com_port_pair() -> PortPair {
    // On Windows, we'll look for existing com0com virtual ports
    // or instruct the user to set them up
    
    println!("Checking for com0com virtual ports...");
    
    // Get available ports
    let ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(e) => {
            println!("Error listing serial ports: {}", e);
            return PortPair {
                client_port: DEFAULT_WINDOWS_CLIENT_PORT.to_string(),
                device_port: DEFAULT_WINDOWS_DEVICE_PORT.to_string(),
            };
        }
    };
    
    // Print all ports for debugging
    println!("All available ports:");
    for port in &ports {
        println!("  - {} (type: {:?})", port.port_name, port.port_type);
    }
    
    // Look for com0com ports (they usually have a specific description)
    let mut com0com_ports = Vec::new();
    for port in &ports {  // Use a reference here instead of moving ports
        if let SerialPortType::UsbPort(info) = &port.port_type {
            println!("  USB port: {} (VID: {:04x}, PID: {:04x}, Manufacturer: {:?})", 
                     port.port_name, info.vid, info.pid, 
                     info.manufacturer.as_ref().map_or("None", |s| s.as_str()));
            
            // Check for com0com in manufacturer or product strings
            let is_com0com = info.manufacturer.as_ref().map_or(false, |m| m.contains("com0com")) ||
                             info.product.as_ref().map_or(false, |p| p.contains("com0com"));
            
            if is_com0com {
                println!("  Found com0com port: {}", port.port_name);
                com0com_ports.push(port.port_name.clone());
            }
        }
    }
    
    if com0com_ports.len() >= 2 {
        // Use the first pair found
        println!("Found com0com virtual ports: {:?}", com0com_ports);
        return PortPair {
            client_port: com0com_ports[0].clone(),
            device_port: com0com_ports[1].clone(),
        };
    }
    
    // If no com0com ports found, check if the default ports exist
    let default_ports_exist = ports.iter().any(|p| p.port_name == DEFAULT_WINDOWS_CLIENT_PORT || p.port_name == DEFAULT_WINDOWS_DEVICE_PORT);
    
    if default_ports_exist {
        println!("Found default COM ports {} and {}", 
                DEFAULT_WINDOWS_CLIENT_PORT, DEFAULT_WINDOWS_DEVICE_PORT);
        return PortPair {
            client_port: DEFAULT_WINDOWS_CLIENT_PORT.to_string(),
            device_port: DEFAULT_WINDOWS_DEVICE_PORT.to_string(),
        };
    }
    
    // If no com0com ports found, instruct the user
    println!("No com0com virtual ports found.");
    println!("Please install com0com from http://com0com.sourceforge.net/");
    println!("and create a virtual port pair (e.g., COM5 and COM6)");
    println!("Using default COM ports {} and {}", DEFAULT_WINDOWS_CLIENT_PORT, DEFAULT_WINDOWS_DEVICE_PORT);
    println!("\nNOTE: If you've already installed com0com and created ports, try:");
    println!("1. Running this simulator as administrator");
    println!("2. Checking Device Manager to confirm the ports exist");
    println!("3. Restarting your computer");
    
    // Return default values
    PortPair {
        client_port: DEFAULT_WINDOWS_CLIENT_PORT.to_string(),
        device_port: DEFAULT_WINDOWS_DEVICE_PORT.to_string(),
    }
}

fn create_unix_com_port_pair() -> PortPair {
    // On Unix systems, we'll create a PTY pair
    // This requires the 'socat' utility to be installed
    
    println!("Creating virtual serial ports using socat...");
    
    // Check if socat is installed
    let socat_check = std::process::Command::new("which")
        .arg("socat")
        .output();
    
    if socat_check.is_err() || !socat_check.unwrap().status.success() {
        println!("socat not found. Please install it:");
        println!("  On Ubuntu/Debian: sudo apt-get install socat");
        println!("  On macOS: brew install socat");
        
        // Return default values
        return PortPair {
            client_port: DEFAULT_UNIX_CLIENT_PORT.to_string(),
            device_port: DEFAULT_UNIX_DEVICE_PORT.to_string(),
        };
    }
    
    // Create a PTY pair using socat
    // This command creates two linked PTYs and prints their names
    let output = std::process::Command::new("socat")
        .arg("-d")
        .arg("-d")
        .arg("pty,raw,echo=0")
        .arg("pty,raw,echo=0")
        .output();
    
    if let Ok(output) = output {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Parse the output to get the PTY names
        // socat outputs something like:
        // 2023/01/01 12:00:00 socat[12345] N PTY is /dev/pts/1
        // 2023/01/01 12:00:00 socat[12345] N PTY is /dev/pts/2
        let mut pty_names = Vec::new();
        for line in stderr.lines() {
            if line.contains("PTY is ") {
                if let Some(pty) = line.split("PTY is ").nth(1) {
                    pty_names.push(pty.trim().to_string());
                }
            }
        }
        
        if pty_names.len() >= 2 {
            return PortPair {
                client_port: pty_names[0].clone(),
                device_port: pty_names[1].clone(),
            };
        }
    }
    
    // If socat failed, return default values
    println!("Failed to create PTY pair with socat.");
    println!("Using default ports {} and {}", DEFAULT_UNIX_CLIENT_PORT, DEFAULT_UNIX_DEVICE_PORT);
    PortPair {
        client_port: DEFAULT_UNIX_CLIENT_PORT.to_string(),
        device_port: DEFAULT_UNIX_DEVICE_PORT.to_string(),
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
        },
        Err(e) => println!("Error listing ports: {}", e),
    }
    
    // Open the serial port with a shorter timeout
    let mut port = match serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(1000))
        .open() {
            Ok(port) => {
                println!("Successfully opened port {}", port_name);
                port
            },
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
                                port.flush().unwrap_or_else(|e| error!("Failed to flush port: {}", e));
                                
                                // Clear the command buffer for the next command
                                command_buffer.clear();
                            }
                        } else {
                            // Add character to command buffer
                            command_buffer.push(c);
                        }
                    }
                }
            },
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



















