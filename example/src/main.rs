use std::sync::mpsc;
use tokio::spawn;
use tokio::sync::broadcast;

use minicircuit::{
    commands::{
        basic::frequency::{GetFrequency, SetFrequency},
        command::Command,
        response::Response,
    },
    drivers::{
        connection::{connect_to_signal_generator, open_port},
        minicircuit_driver::{Message, MiniCircuitDriver, Priority},
        properties::TargetProperties,
    },
};

#[tokio::main]
async fn main() {
    // A channel that will be used by the driver to deliver responses from the commands back.
    let (channel, mut channel_rx) = broadcast::channel::<Response>(100);
    // Spawn a task to continuously receive message responses.
    let handle = spawn(async move {
        while let Ok(response) = channel_rx.recv().await {
            match response {
                Response::GetFrequencyResponse(get_frequency_response) => {
                    println!("Frequency is set to: {}", get_frequency_response.frequency);
                }
                Response::SetFrequencyResponse(set_frequency_response) => {
                    match set_frequency_response.result {
                        Ok(_) => println!("Frequency was sucessfully updated."),
                        Err(e) => println!("An error occurred executing the command. \n{}", e),
                    }
                }
                Response::ReadWriteError(read_write_error) => {
                    println!(
                        "An error occurred sending a command to the signal generator. {}",
                        read_write_error.description
                    )
                }
                Response::MWError(mwerror) => {
                    println!("An error occurred executing a command. {}", mwerror)
                }
                _ => (),
            };
        }
    });

    // A queue that can be used for sending commands to the driver.
    let (queue, queue_rx) = mpsc::channel::<Message>();

    // Connect to the signal generator that has the desired properties.
    let target_properties = TargetProperties::default();
    // The port can either be opened by specifying the physical port using the port property in TargetProperties.
    if false {
        let _port = match open_port(target_properties.clone()) {
            Some(port) => port,
            None => {
                eprintln!("Exiting program: No valid connection.");
                return;
            }
        };
    }
    // Or the port can be automatically detected and opened using the desired product and manufacturer IDs in TargetProperties.
    // Use this method if the port location isn't guaranteed.
    let port = match connect_to_signal_generator(target_properties) {
        Some(port) => port,
        None => {
            eprintln!("Exiting program: No valid connection.");
            return;
        }
    };

    let mut controller = MiniCircuitDriver::new(port, channel, queue_rx);

    // Setter function
    let set_frequency = Command::SetFrequency(SetFrequency::default());
    // Getter function
    let get_frequency = Command::GetFrequency(GetFrequency::default());

    // Giving the "setter" function higher priority so that it is executed before the "getter".
    // This ensures the getter is returning the current state.
    let _ = queue.send(Message {
        priority: Priority::High,
        command: set_frequency.clone(),
    });
    let _ = queue.send(Message {
        priority: Priority::Low,
        command: get_frequency.clone(),
    });

    // Telling the driver to execute all the commands that are in it's queue.
    controller.handle_queue();

    //
    // The driver can also execute commands directly.
    //
    let _set_frequency_response: Response = controller.send(set_frequency);
    let _get_frequency_response: Response = controller.send(get_frequency);

    // Parse the responses from the queue as a result of executing the commands.
    handle.await.unwrap();
}
