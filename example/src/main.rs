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
    let target_properties = TargetProperties::default();
    let mut port = match connect_to_signal_generator(target_properties) {
        Some(port) => port,
        None => {
            eprintln!("Exiting program: No valid connection.");
            return;
        }
    };

    let command = SetFrequency::default();
    let command_string: String = command.clone().into();
    match write_read(&mut *port, command_string) {
        Ok(response) => {
            match SetFrequencyResponse::try_from(response) {
                Ok(_) => println!("Frequency was successfully set to: {}", command.frequency),
                Err(e) => println!("{}", e),
            };
        }
        Err(e) => {
            eprintln!("{:#?}\n\t{}", e.kind, e.description)
        }
    }

    let command = GetFrequency::default();
    let command_string: String = command.clone().into();
    match write_read(&mut *port, command_string) {
        Ok(response) => {
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
