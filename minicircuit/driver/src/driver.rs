use std::sync::{mpsc, Arc};

use serde::{Deserialize, Serialize};
use serialport::{Error, SerialPort};
use tokio::sync::{broadcast, Mutex};

use minicircuit_commands::{
    basic::{
        adc::GetPAPowerADCResponse,
        current::GetPACurrentResponse,
        forward_reflected::{GetPAPowerDBMResponse, GetPAPowerWattResponse},
        frequency::{GetFrequencyResponse, SetFrequencyResponse},
        output::{GetRFOutputResponse, SetRFOutputResponse},
        phase::{GetPhaseResponse, SetPhaseResponse},
        setpoint::{
            GetPAPowerSetpointDBMResponse, GetPAPowerSetpointWattResponse,
            SetPAPowerSetpointDBMResponse, SetPAPowerSetpointWattResponse,
        },
        temperature::GetPATempResponse,
        voltage::GetPAVoltageResponse,
    },
    command::{Command, Message},
    data_types::errors::ReadWriteError,
    dll::{
        config::{GetDLLConfigResponse, SetDLLConfigResponse},
        enable::{GetDLLEnabledResponse, SetDLLEnabledResponse},
        sweep::{PerformSweepDBMResponse, PerformSweepWattResponse},
    },
    error::{
        clear_errors::ClearErrorsResponse, pa::GetPAErrorsResponse, status::GetStatusResponse,
    },
    information::{
        identity::GetIdentityResponse, isc_temp::GetISCTempResponse, uptime::GetUptimeResponse,
        version::GetVersionResponse,
    },
    manual::{
        attenuation::{GetAttenuationResponse, SetAttenuationResponse},
        auto_gain::{GetAutoGainStateResponse, SetAutoGainStateResponse},
        magnitude::{GetMagnitudeResponse, SetMagnitudeResponse},
        power::{GetISCPowerOutputResponse, SetISCPowerOutputResponse},
    },
    pwm::{
        duty_cycle::{GetPWMDutyCycleResponse, SetPWMDutyCycleResponse},
        frequency::SetPWMFrequencyResponse,
        timed_rf::SetTimedRFEnableResponse,
    },
    response::Response,
    soa::{
        config::{GetSOAConfigResponse, SetSOAConfigResponse},
        current::{GetSOACurrentConfigResponse, SetSOACurrentConfigResponse},
        dissipation::{GetSOADissipationConfigResponse, SetSOADissipationConfigResponse},
        forward_power::{GetSOAForwardPowerLimitsResponse, SetSOAForwardPowerLimitsResponse},
        grace_timer::SetSOAGraceTimerResponse,
        reflected_power::{GetSOAPowerConfigResponse, SetSOAPowerConfigResponse},
        temperature::{GetSOATempConfigResponse, SetSOATempConfigResponse},
        voltage::{GetSOAVoltageConfigResponse, SetSOAVoltageConfigResponse},
        watchdog::SetSOAWatchdogConfigResponse,
    },
    system::{
        channel_id::{GetChannelIDResponse, SetChannelIDResponse},
        clock_source::{GetClockSourceResponse, SetClockSourceResponse},
        communication::SetCommunicationInterfaceResponse,
        power_max::{GetPowerMaxDbmResponse, SetPowerMaxDbmResponse},
        power_min::{GetPowerMinDbmResponse, SetPowerMinDbmResponse},
        power_offset::{GetPowerOffsetResponse, SetPowerOffsetResponse},
        system_reset::ResetSystemResponse,
        trigger_delay::SetZHLTriggerDelayResponse,
    },
};

use super::{
    communication::write_read, connection::autodetect_sg_port, properties::TargetProperties,
};

#[derive(Debug)]
pub struct MiniCircuitDriver {
    pub properties: TargetProperties,
    pub queue_handle: Option<tokio::task::JoinHandle<()>>,
}

impl MiniCircuitDriver {
    pub fn new(properties: TargetProperties) -> Self {
        Self {
            properties,
            queue_handle: None,
        }
    }

    pub fn connect(
        &mut self,
    ) -> Result<(mpsc::Sender<Message>, broadcast::Sender<Response>), Error> {
        let properties_clone = self.properties.clone();

        // Try to get a list of ports that match the vendor and product ids
        let signal_generators =
            match autodetect_sg_port(properties_clone.vendor_id, properties_clone.product_id) {
                Ok(list_of_sg) => list_of_sg,
                Err(e) => {
                    // If autodetection fails and we have a specified port, try to use that instead
                    if let Some(port_name) = &properties_clone.port {
                        println!(
                            "Autodetection failed: {}. Falling back to specified port: {}",
                            e, port_name
                        );
                        return self.port_connect();
                    } else {
                        return Err(e);
                    }
                }
            };

        // Verify a port was detected.
        if signal_generators.is_empty() {
            // If no ports were detected but we have a specified port, try to use that instead
            if let Some(port_name) = &properties_clone.port {
                println!("No devices detected matching defined properties. Falling back to specified port: {}", port_name);
                return self.port_connect();
            } else {
                return Err(Error::new(
                    serialport::ErrorKind::NoDevice,
                    "Unable to detect device matching defined properties.",
                ));
            }
        }

        // Connect to the first port that matches the requirements.
        let first_signal_generator = &signal_generators[0];

        // Open a serial connection with the detected port at the requested settings.
        let port = match serialport::new(
            &first_signal_generator.port_name,
            properties_clone.baud_rate.into(),
        )
        .data_bits(properties_clone.data_bits)
        .parity(properties_clone.parity)
        .flow_control(properties_clone.flow_control)
        .stop_bits(properties_clone.stop_bits)
        .timeout(properties_clone.connection_timeout)
        .open()
        {
            Ok(port) => port,
            Err(e) => {
                return Err(e);
            }
        };

        // Wrap `port` in `Arc<Mutex<T>>` so it can be shared across threads.
        let port = Arc::new(Mutex::new(port));

        // Create a channel that will be used by the driver to deliver responses from the commands back to the caller.
        let (channel_tx, channel_rx) = broadcast::channel::<Response>(100);
        // Create a queue that can be used by the driver for receiving commands.
        let (queue_tx, queue_rx) = mpsc::channel::<Message>();

        // Clone Arc pointers for the thread.
        let port_clone = Arc::clone(&port);

        // Spawn a thread for handling commands in the queue.
        // Store the handle so the thread doesn't get dropped.
        self.queue_handle = Some(spawn_queue_loop(queue_rx, port_clone, channel_tx.clone()));

        // Return the queue sender and response receiver.
        Ok((queue_tx, channel_rx))
    }

    pub fn port_connect(
        &mut self,
    ) -> Result<(mpsc::Sender<Message>, broadcast::Sender<Response>), Error> {
        let properties_clone = self.properties.clone();

        let Some(port_name) = properties_clone.port else {
            return Err(Error::new(serialport::ErrorKind::InvalidInput, "A port must be defined in order to connect to it. Please add a port to the target properties."));
        };

        // Open a serial connection with the detected port at the requested settings.
        let port = match serialport::new(port_name.clone(), properties_clone.baud_rate.into())
            .data_bits(properties_clone.data_bits)
            .parity(properties_clone.parity)
            .flow_control(properties_clone.flow_control)
            .stop_bits(properties_clone.stop_bits)
            .timeout(properties_clone.connection_timeout)
            .open()
        {
            Ok(port) => port,
            Err(e) => {
                return Err(e);
            }
        };

        // Wrap `port` in `Arc<Mutex<T>>` so it can be shared across threads.
        let port = Arc::new(Mutex::new(port));

        // Create a channel that will be used by the driver to deliver responses from the commands back to the caller.
        let (channel_tx, channel_rx) = broadcast::channel::<Response>(100);
        // Create a queue that can be used by the driver for receiving commands.
        let (queue_tx, queue_rx) = mpsc::channel::<Message>();

        // Clone Arc pointers for the thread.
        let port_clone = Arc::clone(&port);

        // Spawn a thread for handling commands in the queue.
        // Store the handle so the thread doesn't get dropped
        self.queue_handle = Some(spawn_queue_loop(queue_rx, port_clone, channel_tx.clone()));

        // Return the queue sender and response receiver.
        Ok((queue_tx, channel_tx))
    }
}

fn spawn_queue_loop(
    queue_rx: std::sync::mpsc::Receiver<Message>,
    port: Arc<tokio::sync::Mutex<Box<dyn SerialPort>>>,
    channel_tx: tokio::sync::broadcast::Sender<Response>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            // Define a vector for the queue so that it can be manipulated freely.
            let mut queue = Vec::new();
            while let Ok(msg) = queue_rx.try_recv() {
                queue.push(msg);
            }

            // Sort the messages in the queue by priority.
            queue.sort_by(|a, b| b.priority.cmp(&a.priority));

            // Loop through the messages in the queue.
            for message in queue {
                // Send the command to the controller and wait for the response.
                let response = {
                    let mut port = port.lock().await;
                    send_command(message.command, &mut **port)
                };

                // Return the response to the caller.
                let _ = channel_tx.send(response);
            }

            // Rest for the CPU.
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    })
}

#[allow(deprecated)]
fn send_command(command: Command, port: &mut dyn SerialPort) -> Response {
    match command {
        Command::GetPAPowerADC(get_papower_adc) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_papower_adc.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPAPowerADCResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPAPowerADCResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetPAPowerADC(get_papower_adc), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPACurrent(get_pacurrent) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_pacurrent.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPACurrentResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPACurrentResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetPACurrent(get_pacurrent), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPAPowerDBM(get_papower_dbm) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_papower_dbm.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPAPowerDBMResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPAPowerDBMResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetPAPowerDBM(get_papower_dbm), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPAPowerWatt(get_papower_watt) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_papower_watt.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPAPowerWattResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPAPowerWattResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetPAPowerWatt(get_papower_watt),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetFrequency(get_frequency) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_frequency.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetFrequencyResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetFrequencyResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetFrequency(get_frequency), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetFrequency(set_frequency) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_frequency.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetFrequencyResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetFrequencyResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::SetFrequency(set_frequency), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetRFOutput(get_rfoutput) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_rfoutput.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetRFOutputResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetRFOutputResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetRFOutput(get_rfoutput), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetRFOutput(set_rfoutput) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_rfoutput.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetRFOutputResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::SetRFOutputResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::SetRFOutput(set_rfoutput), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPhase(get_phase) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_phase.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPhaseResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetPhaseResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetPhase(get_phase), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetPhase(set_phase) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_phase.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetPhaseResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::SetPhaseResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::SetPhase(set_phase), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPAPowerSetpointDBM(get_papower_setpoint_dbm) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_papower_setpoint_dbm.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPAPowerSetpointDBMResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPAPowerSetpointDBMResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetPAPowerSetpointDBM(get_papower_setpoint_dbm),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPAPowerSetpointWatt(get_papower_setpoint_watt) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_papower_setpoint_watt.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPAPowerSetpointWattResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPAPowerSetpointWattResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetPAPowerSetpointWatt(get_papower_setpoint_watt),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetPAPowerSetpointDBM(set_papower_setpoint_dbm) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_papower_setpoint_dbm.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetPAPowerSetpointDBMResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetPAPowerSetpointDBMResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetPAPowerSetpointDBM(set_papower_setpoint_dbm),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetPAPowerSetpointWatt(set_papower_setpoint_watt) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_papower_setpoint_watt.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetPAPowerSetpointWattResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetPAPowerSetpointWattResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetPAPowerSetpointWatt(set_papower_setpoint_watt),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPATemp(get_patemp) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_patemp.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPATempResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetPATempResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetPATemp(get_patemp), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPAVoltage(get_pavoltage) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_pavoltage.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPAVoltageResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPAVoltageResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetPAVoltage(get_pavoltage), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetDLLConfig(get_dllconfig) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_dllconfig.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetDLLConfigResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetDLLConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetDLLConfig(get_dllconfig), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetDLLConfig(set_dllconfig) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_dllconfig.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetDLLConfigResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetDLLConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::SetDLLConfig(set_dllconfig), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetDLLEnabled(get_dllenabled) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_dllenabled.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetDLLEnabledResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetDLLEnabledResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetDLLEnabled(get_dllenabled), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetDLLEnabled(set_dllenabled) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_dllenabled.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetDLLEnabledResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetDLLEnabledResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::SetDLLEnabled(set_dllenabled), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::PerformSweepDBM(perform_sweep_dbm) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = perform_sweep_dbm.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<PerformSweepDBMResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::PerformSweepDBMResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::PerformSweepDBM(perform_sweep_dbm),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::PerformSweepWatt(perform_sweep_watt) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = perform_sweep_watt.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<PerformSweepWattResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::PerformSweepWattResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::PerformSweepWatt(perform_sweep_watt),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::ClearErrors(clear_errors) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = clear_errors.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<ClearErrorsResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::ClearErrorsResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::ClearErrors(clear_errors), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPAErrors(get_paerrors) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_paerrors.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPAErrorsResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetPAErrorsResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetPAErrors(get_paerrors), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetStatus(get_status) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_status.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetStatusResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetStatusResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetStatus(get_status), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetIdentity(get_identity) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_identity.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetIdentityResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetIdentityResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetIdentity(get_identity), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetISCTemp(get_isctemp) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_isctemp.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetISCTempResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetISCTempResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetISCTemp(get_isctemp), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetUptime(get_uptime) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_uptime.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetUptimeResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetUptimeResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetUptime(get_uptime), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetVersion(get_version) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_version.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetVersionResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::GetVersionResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetVersion(get_version), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetAttenuation(get_attenuation) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_attenuation.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetAttenuationResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetAttenuationResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetAttenuation(get_attenuation),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetAttenuation(set_attenuation) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_attenuation.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetAttenuationResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetAttenuationResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetAttenuation(set_attenuation),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetAutoGainState(get_auto_gain_state) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_auto_gain_state.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetAutoGainStateResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetAutoGainStateResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetAutoGainState(get_auto_gain_state),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetAutoGainState(set_auto_gain_state) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_auto_gain_state.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetAutoGainStateResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetAutoGainStateResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetAutoGainState(set_auto_gain_state),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetMagnitude(get_magnitude) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_magnitude.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetMagnitudeResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetMagnitudeResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetMagnitude(get_magnitude), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetMagnitude(set_magnitude) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_magnitude.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetMagnitudeResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetMagnitudeResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::SetMagnitude(set_magnitude), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetISCPowerOutput(get_iscpower_output) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_iscpower_output.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetISCPowerOutputResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetISCPowerOutputResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetISCPowerOutput(get_iscpower_output),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetISCPowerOutput(set_iscpower_output) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_iscpower_output.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetISCPowerOutputResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetISCPowerOutputResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetISCPowerOutput(set_iscpower_output),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPWMDutyCycle(get_pwmduty_cycle) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_pwmduty_cycle.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPWMDutyCycleResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPWMDutyCycleResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetPWMDutyCycle(get_pwmduty_cycle),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetPWMDutyCycle(set_pwmduty_cycle) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_pwmduty_cycle.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetPWMDutyCycleResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetPWMDutyCycleResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetPWMDutyCycle(set_pwmduty_cycle),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetPWMFrequency(set_pwmfrequency) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_pwmfrequency.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetPWMFrequencyResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetPWMFrequencyResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetPWMFrequency(set_pwmfrequency),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetTimedRFEnable(set_timed_rfenable) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_timed_rfenable.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetTimedRFEnableResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetTimedRFEnableResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetTimedRFEnable(set_timed_rfenable),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetSOAConfig(get_soaconfig) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_soaconfig.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetSOAConfigResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetSOAConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetSOAConfig(get_soaconfig), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOAConfig(set_soaconfig) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soaconfig.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOAConfigResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOAConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::SetSOAConfig(set_soaconfig), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetSOACurrentConfig(get_soacurrent_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_soacurrent_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetSOACurrentConfigResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetSOACurrentConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetSOACurrentConfig(get_soacurrent_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOACurrentConfig(set_soacurrent_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soacurrent_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOACurrentConfigResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOACurrentConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetSOACurrentConfig(set_soacurrent_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetSOADissipationConfig(get_soadissipation_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_soadissipation_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetSOADissipationConfigResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetSOADissipationConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetSOADissipationConfig(get_soadissipation_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOADissipationConfig(set_soadissipation_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soadissipation_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOADissipationConfigResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOADissipationConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetSOADissipationConfig(set_soadissipation_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetSOAForwardPowerLimits(get_soaforward_power_limits) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_soaforward_power_limits.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetSOAForwardPowerLimitsResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetSOAForwardPowerLimitsResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetSOAForwardPowerLimits(get_soaforward_power_limits),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOAForwardPowerLimits(set_soaforward_power_limits) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soaforward_power_limits.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOAForwardPowerLimitsResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOAForwardPowerLimitsResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetSOAForwardPowerLimits(set_soaforward_power_limits),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOAGraceTimer(set_soagrace_timer) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soagrace_timer.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOAGraceTimerResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOAGraceTimerResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetSOAGraceTimer(set_soagrace_timer),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetSOAPowerConfig(get_soapower_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_soapower_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetSOAPowerConfigResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetSOAPowerConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetSOAPowerConfig(get_soapower_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOAPowerConfig(set_soapower_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soapower_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOAPowerConfigResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOAPowerConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetSOAPowerConfig(set_soapower_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetSOATempConfig(get_soatemp_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_soatemp_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetSOATempConfigResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetSOATempConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetSOATempConfig(get_soatemp_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOATempConfig(set_soatemp_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soatemp_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOATempConfigResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOATempConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetSOATempConfig(set_soatemp_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetSOAVoltageConfig(get_soavoltage_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_soavoltage_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetSOAVoltageConfigResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetSOAVoltageConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetSOAVoltageConfig(get_soavoltage_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOAVoltageConfig(set_soavoltage_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soavoltage_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOAVoltageConfigResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOAVoltageConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetSOAVoltageConfig(set_soavoltage_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetSOAWatchdogConfig(set_soawatchdog_config) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_soawatchdog_config.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetSOAWatchdogConfigResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetSOAWatchdogConfigResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetSOAWatchdogConfig(set_soawatchdog_config),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetUartBaudRate(set_uart_baud_rate) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_uart_baud_rate.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                // This command doesn't have a response from the signal generator.
                Ok(_) => Response::SetUartBaudRate,
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetUartBaudRate(set_uart_baud_rate),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetChannelID(get_channel_id) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_channel_id.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetChannelIDResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetChannelIDResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::GetChannelID(get_channel_id), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetChannelID(set_channel_id) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_channel_id.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetChannelIDResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetChannelIDResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::SetChannelID(set_channel_id), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetClockSource(get_clock_source) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_clock_source.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetClockSourceResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetClockSourceResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetClockSource(get_clock_source),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetClockSource(set_clock_source) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_clock_source.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetClockSourceResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetClockSourceResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetClockSource(set_clock_source),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetCommunicationInterface(set_communication_interface) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_communication_interface.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetCommunicationInterfaceResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetCommunicationInterfaceResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetCommunicationInterface(set_communication_interface),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPowerMaxDbm(get_power_max_dbm) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_power_max_dbm.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPowerMaxDbmResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPowerMaxDbmResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetPowerMaxDbm(get_power_max_dbm),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetPowerMaxDbm(set_power_max_dbm) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_power_max_dbm.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetPowerMaxDbmResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetPowerMaxDbmResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetPowerMaxDbm(set_power_max_dbm),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPowerMinDbm(get_power_min_dbm) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_power_min_dbm.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPowerMinDbmResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPowerMinDbmResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetPowerMinDbm(get_power_min_dbm),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetPowerMinDbm(set_power_min_dbm) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_power_min_dbm.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetPowerMinDbmResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetPowerMinDbmResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetPowerMinDbm(set_power_min_dbm),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::GetPowerOffset(get_power_offset) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = get_power_offset.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<GetPowerOffsetResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::GetPowerOffsetResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::GetPowerOffset(get_power_offset),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetPowerOffset(set_power_offset) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_power_offset.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetPowerOffsetResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetPowerOffsetResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetPowerOffset(set_power_offset),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::ResetSystem(reset_system) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = reset_system.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<ResetSystemResponse, _> = sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => Response::ResetSystemResponse(formatted_response),
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response =
                        ReadWriteError::new(Command::ResetSystem(reset_system), e.description);

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
        Command::SetZHLTriggerDelay(set_zhltrigger_delay) => {
            // Convert the command into a string (required format to be sent to the signal generator).
            let command: String = set_zhltrigger_delay.clone().into();

            // Collect the resulting response of sending the command.
            let command_response: Response = match write_read(port, command) {
                Ok(sg_response) => {
                    let parse_result: Result<SetZHLTriggerDelayResponse, _> =
                        sg_response.try_into();

                    match parse_result {
                        Ok(formatted_response) => {
                            Response::SetZHLTriggerDelayResponse(formatted_response)
                        }
                        Err(e) => Response::MWError(e),
                    }
                }
                // Return the command (for backtracking the source of issue) and the error description
                Err(e) => {
                    let error_response = ReadWriteError::new(
                        Command::SetZHLTriggerDelay(set_zhltrigger_delay),
                        e.description,
                    );

                    Response::ReadWriteError(error_response)
                }
            };

            // Directly return the response to the caller rather than sending it to a queue.
            command_response
        }
    }
}
