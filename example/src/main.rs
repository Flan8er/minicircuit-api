use minicircuit::{
    commands::basic::frequency::{
        GetFrequency, GetFrequencyResponse, SetFrequency, SetFrequencyResponse,
    },
    drivers::{
        communication::write_read, connection::connect_to_signal_generator,
        properties::TargetProperties,
    },
};

fn main() {
    // Connect to the signal generator that has the desired properties.
    let target_properties = TargetProperties::default();
    let mut port = match connect_to_signal_generator(target_properties) {
        Some(port) => port,
        None => {
            eprintln!("Exiting program: No valid connection.");
            return;
        }
    };

    // Create and send a command to set the frequency of the signal generator.
    let command = SetFrequency::default();
    let command_string: String = command.clone().into();
    match write_read(&mut *port, command_string) {
        // Parse the response from sending the command.
        Ok(response) => {
            // Every "set" command has a response of either being successful or the error that occured.
            match SetFrequencyResponse::try_from(response) {
                Ok(_) => println!("Frequency was successfully set to: {}", command.frequency),
                Err(e) => println!("{}", e),
            };
        }
        Err(e) => {
            eprintln!("{:#?}\n\t{}", e.kind, e.description)
        }
    }

    // Create and send a command to get the frequency the signal generator is set to.
    let command = GetFrequency::default();
    let command_string: String = command.clone().into();
    match write_read(&mut *port, command_string) {
        // Parse the response from sending the command.
        Ok(response) => {
            // Every "get" command responds with the desired information or the error that occured.
            match GetFrequencyResponse::try_from(response) {
                Ok(response) => println!("Frequency is currently set to: {}", response.frequency),
                Err(e) => println!("{}", e),
            };
        }
        Err(e) => {
            eprintln!("{:#?}\n\t{}", e.kind, e.description)
        }
    }
}
