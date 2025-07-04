use tokio::spawn;

use minicircuit_commands::properties::TargetProperties;
use minicircuit_commands::{
    basic::frequency::{GetFrequency, SetFrequency},
    command::{Command, Message, Priority},
};
use minicircuit_driver::{connection::print_available_ports, driver::MiniCircuitDriver};

#[tokio::main]
async fn main() {
    print_available_ports();

    // Define the properties of the signal generator you are working with.
    let target_properties = TargetProperties::default();

    // Build the controller driver
    let mut controller = MiniCircuitDriver::new(target_properties);

    // The port can either be opened by specifying the physical port using the port property in TargetProperties.
    if false {
        let (_channel_tx, _log) = match controller.port_connect() {
            Ok(channels) => channels,
            Err(e) => {
                eprintln!("Unable to connect to the controller: {}", e);
                return;
            }
        };
    }
    // Or the port can be automatically detected and opened using the desired product and manufacturer IDs in TargetProperties.
    // Use this method if the port location isn't guaranteed.
    let (channel_tx, response_tx) = match controller.connect() {
        Ok(channels) => channels,
        Err(e) => {
            eprintln!("Unable to connect to the controller: {}", e);
            return;
        }
    };

    let handle = spawn(async move {
        let mut rx = response_tx.subscribe();
        while let Ok(response) = rx.recv().await {
            let response: String = response.into();
            println!("{}", response);
        }
    });

    // Setter function
    let set_frequency = Command::SetFrequency(SetFrequency::default());
    // Getter function
    let get_frequency = Command::GetFrequency(GetFrequency::default());

    // Giving the "setter" function higher priority so that it is executed before the "getter".
    // This ensures the getter is returning the current state.
    let _ = channel_tx.send(Message {
        priority: Priority::High,
        command: set_frequency.clone(),
    });
    let _ = channel_tx.send(Message {
        priority: Priority::Low,
        command: get_frequency.clone(),
    });

    handle.await.unwrap();
}
