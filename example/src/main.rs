use std::sync::{mpsc, Arc};
use tokio::sync::broadcast;

use minicircuit::{
    commands::{
        basic::frequency::{
            GetFrequency, GetFrequencyResponse, SetFrequency, SetFrequencyResponse,
        },
        commands::Command,
    },
    drivers::{
        communication::write_read,
        connection::connect_to_signal_generator,
        minicircuit_driver::{Message, MiniCircuitDriver, Priority},
        properties::TargetProperties,
    },
};

fn main() {
    // A channel that will be used by the driver to deliver messages back.
    let (channel, channel_rx) = broadcast::channel::<String>(100);

    // A queue that can be used for sending messages.
    let (queue, queue_rx) = mpsc::channel::<Message>();

    // Connect to the signal generator that has the desired properties.
    let target_properties = TargetProperties::default();
    let mut port = match connect_to_signal_generator(target_properties) {
        Some(port) => port,
        None => {
            eprintln!("Exiting program: No valid connection.");
            return;
        }
    };

    let controller = Arc::new(MiniCircuitDriver { port, channel });

    queue.send(Message {
        priority: Priority::Low,
        command: Command::SetFrequency(SetFrequency::default()),
    });
    queue.send(Message {
        priority: Priority::Low,
        command: Command::GetFrequency(GetFrequency::default()),
    });

    // Go through the queue sending the commands to the driver
    queue_rx.iter().for_each(move |message| {
        let message = message.clone();

        match message.clone().command {
            Command::GetPAPowerADC(get_papower_adc) => todo!(),
            Command::GetPACurrent(get_pacurrent) => todo!(),
            Command::GetPAPowerDBM(get_papower_dbm) => todo!(),
            Command::GetPAPowerWatt(get_papower_watt) => todo!(),
            Command::GetFrequency(get_frequency) => {
                match controller.clone().send(message.command) {
                    Ok(response) => {
                        // Every "get" command responds with the desired information or the error that occured.
                        let response = match GetFrequencyResponse::try_from(response) {
                            Ok(response) => {
                                format!("Frequency is currently set to: {}", response.frequency)
                            }
                            Err(e) => format!("{}", e),
                        };

                        // Push the response to the queue.
                        channel.clone().send(response);
                    }
                    Err(e) => eprintln!("{:#?}\n\t{}", e.kind, e.description),
                };
            }
            Command::SetFrequency(set_frequency) => {
                match controller.clone().send(message.clone().command) {
                    Ok(response) => {
                        // Every "set" command has a response of either being successful or the error that occured.
                        let response = match SetFrequencyResponse::try_from(response) {
                            Ok(_) => String::from("Frequency was sucessfully updated"),
                            Err(e) => format!("{}", e),
                        };

                        // Push the response to the queue.
                        channel.clone().send(response);
                    }
                    Err(e) => eprintln!("{:#?}\n\t{}", e.kind, e.description),
                };
            }
            Command::GetRFOutput(get_rfoutput) => todo!(),
            Command::SetRFOutput(set_rfoutput) => todo!(),
            Command::GetPhase(get_phase) => todo!(),
            Command::SetPhase(set_phase) => todo!(),
            Command::GetPAPowerSetpointDBM(get_papower_setpoint_dbm) => todo!(),
            Command::GetPAPowerSetpointWatt(get_papower_setpoint_watt) => todo!(),
            Command::SetPAPowerSetpointDBM(set_papower_setpoint_dbm) => todo!(),
            Command::SetPAPowerSetpointWatt(set_papower_setpoint_watt) => todo!(),
            Command::GetPATemp(get_patemp) => todo!(),
            Command::GetPAVoltage(get_pavoltage) => todo!(),
            Command::GetDLLConfig(get_dllconfig) => todo!(),
            Command::SetDLLConfig(set_dllconfig) => todo!(),
            Command::GetDLLEnabled(get_dllenabled) => todo!(),
            Command::SetDLLEnabled(set_dllenabled) => todo!(),
            Command::PerformSweepDBM(perform_sweep_dbm) => todo!(),
            Command::PerformSweepWatt(perform_sweep_watt) => todo!(),
            Command::ClearErrors(clear_errors) => todo!(),
            Command::GetPAErrors(get_paerrors) => todo!(),
            Command::GetStatus(get_status) => todo!(),
            Command::GetIdentity(get_identity) => todo!(),
            Command::GetISCTemp(get_isctemp) => todo!(),
            Command::GetUptime(get_uptime) => todo!(),
            Command::GetVersion(get_version) => todo!(),
            Command::GetAttenuation(get_attenuation) => todo!(),
            Command::SetAttenuation(set_attenuation) => todo!(),
            Command::GetAutoGainState(get_auto_gain_state) => todo!(),
            Command::SetAutoGainState(set_auto_gain_state) => todo!(),
            Command::GetMagnitude(get_magnitude) => todo!(),
            Command::SetMagnitude(set_magnitude) => todo!(),
            Command::GetISCPowerOutput(get_iscpower_output) => todo!(),
            Command::SetISCPowerOutput(set_iscpower_output) => todo!(),
            Command::GetPWMDutyCycle(get_pwmduty_cycle) => todo!(),
            Command::SetPWMDutyCycle(set_pwmduty_cycle) => todo!(),
            Command::SetPWMFrequency(set_pwmfrequency) => todo!(),
            Command::SetTimedRFEnable(set_timed_rfenable) => todo!(),
            Command::GetSOAConfig(get_soaconfig) => todo!(),
            Command::SetSOAConfig(set_soaconfig) => todo!(),
            Command::GetSOACurrentConfig(get_soacurrent_config) => todo!(),
            Command::SetSOACurrentConfig(set_soacurrent_config) => todo!(),
            Command::GetSOADissipationConfig(get_soadissipation_config) => todo!(),
            Command::SetSOADissipationConfig(set_soadissipation_config) => todo!(),
            Command::GetSOAForwardPowerLimits(get_soaforward_power_limits) => todo!(),
            Command::SetSOAForwardPowerLimits(set_soaforward_power_limits) => todo!(),
            Command::SetSOAGraceTimer(set_soagrace_timer) => todo!(),
            Command::GetSOAPowerConfig(get_soapower_config) => todo!(),
            Command::SetSOAPowerConfig(set_soapower_config) => todo!(),
            Command::GetSOATempConfig(get_soatemp_config) => todo!(),
            Command::SetSOATempConfig(set_soatemp_config) => todo!(),
            Command::GetSOAVoltageConfig(get_soavoltage_config) => todo!(),
            Command::SetSOAVoltageConfig(set_soavoltage_config) => todo!(),
            Command::SetSOAWatchdogConfig(set_soawatchdog_config) => todo!(),
            Command::SetUartBaudRate(set_uart_baud_rate) => todo!(),
            Command::GetChannelID(get_channel_id) => todo!(),
            Command::SetChannelID(set_channel_id) => todo!(),
            Command::GetClockSource(get_clock_source) => todo!(),
            Command::SetClockSource(set_clock_source) => todo!(),
            Command::SetCommunicationInterface(set_communication_interface) => todo!(),
            Command::GetPowerMaxDbm(get_power_max_dbm) => todo!(),
            Command::SetPowerMaxDbm(set_power_max_dbm) => todo!(),
            Command::GetPowerMinDbm(get_power_min_dbm) => todo!(),
            Command::SetPowerMinDbm(set_power_min_dbm) => todo!(),
            Command::GetPowerOffset(get_power_offset) => todo!(),
            Command::SetPowerOffset(set_power_offset) => todo!(),
            Command::ResetSystem(reset_system) => todo!(),
            Command::SetZHLTriggerDelay(set_zhltrigger_delay) => todo!(),
        };
    })
}

// // Create and send a command to set the frequency of the signal generator.
// let command = SetFrequency::default();
// let command_string: String = command.clone().into();
// match write_read(&mut *port, command_string) {
//     // Parse the response from sending the command.
//     Ok(response) => {
//         // Every "set" command has a response of either being successful or the error that occured.
//         match SetFrequencyResponse::try_from(response) {
//             Ok(_) => println!("Frequency was successfully set to: {}", command.frequency),
//             Err(e) => println!("{}", e),
//         };
//     }
//     Err(e) => {
//         eprintln!("{:#?}\n\t{}", e.kind, e.description)
//     }
// }

// // Create and send a command to get the frequency the signal generator is set to.
// let command = GetFrequency::default();
// let command_string: String = command.clone().into();
// match write_read(&mut *port, command_string) {
//     // Parse the response from sending the command.
//     Ok(response) => {
//         // Every "get" command responds with the desired information or the error that occured.
//         match GetFrequencyResponse::try_from(response) {
//             Ok(response) => println!("Frequency is currently set to: {}", response.frequency),
//             Err(e) => println!("{}", e),
//         };
//     }
//     Err(e) => {
//         eprintln!("{:#?}\n\t{}", e.kind, e.description)
//     }
// }
// }
