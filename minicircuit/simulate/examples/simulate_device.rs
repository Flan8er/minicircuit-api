use std::time::Duration;

use minicircuit_commands::{prelude::*, properties::TargetProperties};
use minicircuit_driver::driver::MiniCircuitDriver;
use tokio::runtime::Runtime;

fn main() {
    // Initialize logger
    env_logger::init();

    println!("Starting MiniCircuit simulator example");

    // Create a Tokio runtime
    let rt = Runtime::new().unwrap();

    // Execute the async code within the runtime
    rt.block_on(async {
        // Run the simulator in a separate process first using:
        // cargo run --bin minicircuit-simulator

        // The simulator will print the client port name to connect to
        // Use that port name here:
        let port_name = "COM5"; // Replace with the port name printed by the simulator

        // Create a driver that will connect to the simulator
        let mut target_properties = TargetProperties::default();
        target_properties.port = Some(port_name.to_string());

        let mut driver = MiniCircuitDriver::new(target_properties);

        // Connect to the simulator
        match driver.connect() {
            Ok((tx, mut rx)) => {
                println!("Connected to simulator on port {}", port_name);

                // Add a delay after connecting to ensure the simulator is ready
                tokio::time::sleep(Duration::from_millis(1000)).await;

                // Define test commands
                let test_commands = vec![
                    Command::GetFrequency(GetFrequency::default()),
                    Command::SetFrequency(SetFrequency::new(Channel::default(), 2500.into())),
                    Command::GetFrequency(GetFrequency::default()),
                    Command::SetRFOutput(SetRFOutput::new(Channel::default(), true)),
                    Command::GetRFOutput(GetRFOutput::default()),
                    Command::GetIdentity(GetIdentity::default()),
                    Command::GetISCTemp(GetISCTemp::default()),
                    Command::GetUptime(GetUptime::default()),
                    Command::GetStatus(GetStatus::default()),
                ];

                // Send each command and wait for its response before sending the next one
                for cmd in test_commands {
                    println!("Sending: {:?}", cmd);

                    // Clear the channel before sending a new command
                    while let Ok(_) = rx.try_recv() {
                        // Discard any pending responses
                    }

                    // Send the command with standard priority
                    tx.send(Message {
                        command: cmd.clone(),
                        priority: Priority::Standard,
                    })
                    .unwrap();

                    // Wait for the response with timeout
                    let mut response_received = false;
                    for attempt in 1..=10 {
                        tokio::time::sleep(Duration::from_millis(100)).await;

                        if let Ok(response) = rx.try_recv() {
                            println!("Response (attempt {}): {:?}", attempt, response);
                            response_received = true;
                            break;
                        }
                    }

                    if !response_received {
                        println!("No response received after 10 attempts");
                    }

                    // Add a small delay between commands
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
            Err(e) => {
                println!("Failed to connect to simulator: {}", e);
                println!("Make sure the simulator is running and the port name is correct");
            }
        }
    });
}
