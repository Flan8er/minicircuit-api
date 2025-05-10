use std::sync::{Arc, Mutex};
use std::time::Duration;

use minicircuit_commands::{
    basic::frequency::{GetFrequency, SetFrequency},
    basic::output::{GetRFOutput, SetRFOutput},
    command::{Command, Message, Priority},
};
use minicircuit_commands::data_types::types::Channel;
use minicircuit_driver::{
    driver::MiniCircuitDriver,
    properties::TargetProperties,
    communication::write_read,
};
use serialport::SerialPort;

fn main() {
    // Initialize logger
    env_logger::init();
    
    println!("Starting MiniCircuit simulator example");
    
    // Run the simulator in a separate process first using:
    // cargo run --bin minicircuit-simulator
    
    // The simulator will print the client port name to connect to
    // Use that port name here:
    let port_name = "COM4"; // Replace with the port name printed by the simulator
    
    // Create a driver that will connect to the simulator
    let mut target_properties = TargetProperties::default();
    target_properties.port = Some(port_name.to_string());
    
    let mut driver = MiniCircuitDriver::new(target_properties);
    
    // Connect to the simulator using port_connect()
    match driver.port_connect() {
        Ok((tx, mut rx)) => {
            println!("Connected to simulator on port {}", port_name);
            
            // Send some test commands
            let commands = vec![
                Message {
                    command: Command::GetFrequency(GetFrequency::default()),
                    priority: Priority::Standard,
                },
                Message {
                    command: Command::SetFrequency(SetFrequency::new(Channel::default(), 2500.into())),
                    priority: Priority::Standard,
                },
                Message {
                    command: Command::GetFrequency(GetFrequency::default()),
                    priority: Priority::Standard,
                },
                Message {
                    command: Command::SetRFOutput(SetRFOutput::new(Channel::default(), true)),
                    priority: Priority::Standard,
                },
                Message {
                    command: Command::GetRFOutput(GetRFOutput::default()),
                    priority: Priority::Standard,
                },
            ];
            
            // Send each command and wait for response
            for cmd in commands {
                println!("Sending: {:?}", cmd.command);
                
                // Send the command
                tx.send(cmd).unwrap();
                
                // Wait for the response
                std::thread::sleep(Duration::from_millis(100));
                
                // Get the response
                if let Ok(response) = rx.try_recv() {
                    println!("Response: {:?}", response);
                } else {
                    println!("No response received");
                }
                
                // Add a small delay between commands
                std::thread::sleep(Duration::from_millis(100));
            }
        },
        Err(e) => {
            println!("Failed to connect to simulator: {}", e);
            println!("Make sure the simulator is running and the port name is correct");
        }
    }
}
