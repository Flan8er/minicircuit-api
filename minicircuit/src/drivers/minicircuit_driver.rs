use serde::{Deserialize, Serialize};
use serialport::SerialPort;

use crate::commands::{
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
    command::Command,
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

use super::communication::{write_read, ReadWriteError};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Standard,
    High,
    Immediate,
    Termination,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub priority: Priority,
    pub command: Command,
}

#[derive(Debug)]
pub struct MiniCircuitDriver {
    pub port: Box<dyn SerialPort>,
    pub channel: tokio::sync::broadcast::Sender<Response>,
    pub queue: std::sync::mpsc::Receiver<Message>,
}

impl MiniCircuitDriver {
    pub fn new(
        port: Box<dyn SerialPort>,
        channel: tokio::sync::broadcast::Sender<Response>,
        queue: std::sync::mpsc::Receiver<Message>,
    ) -> Self {
        Self {
            port,
            channel,
            queue,
        }
    }

    /// This allows directly sending commands to the signal generator (skipping the need for the queue.)
    ///
    /// Crutial when one command directly depends on another happening first.
    /// Ex. SetFrequency then GetFrequency (to verify frequency was correctly set.)
    ///
    /// Response of the command will be directly delivered back.
    pub fn send(&mut self, command: Command) -> Response {
        match command {
            Command::GetPAPowerADC(get_papower_adc) => {
                // Convert the command into a string (required format to be sent to the signal generator).
                let command: String = get_papower_adc.clone().into();

                // Collect the resulting response of sending the command.
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetPAPowerADC(get_papower_adc),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetPACurrent(get_pacurrent),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetPAPowerDBM(get_papower_dbm),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetPAPowerWattResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetFrequency(get_frequency),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::SetFrequency(set_frequency),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetRFOutputResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetRFOutputResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetRFOutput(get_rfoutput),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetRFOutputResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::SetRFOutputResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::SetRFOutput(set_rfoutput),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetPhaseResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetPhaseResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetPhase(get_phase),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetPhaseResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::SetPhaseResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::SetPhase(set_phase),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetPATempResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetPATempResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetPATemp(get_patemp),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetPAVoltage(get_pavoltage),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetDLLConfig(get_dllconfig),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::SetDLLConfig(set_dllconfig),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetDLLEnabled(get_dllenabled),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::SetDLLEnabled(set_dllenabled),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<PerformSweepDBMResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<PerformSweepWattResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<ClearErrorsResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::ClearErrorsResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::ClearErrors(clear_errors),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetPAErrorsResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetPAErrorsResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetPAErrors(get_paerrors),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetStatusResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetStatusResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetStatus(get_status),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetIdentityResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetIdentityResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetIdentity(get_identity),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetISCTempResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetISCTempResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetISCTemp(get_isctemp),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetUptimeResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetUptimeResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetUptime(get_uptime),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetVersionResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::GetVersionResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::GetVersion(get_version),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetAttenuationResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetAttenuationResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetAutoGainStateResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetAutoGainStateResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetMagnitude(get_magnitude),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::SetMagnitude(set_magnitude),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetISCPowerOutputResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetISCPowerOutputResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetPWMDutyCycleResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetPWMDutyCycleResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetPWMFrequencyResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetTimedRFEnableResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetSOAConfig(get_soaconfig),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::SetSOAConfig(set_soaconfig),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetSOAGraceTimerResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetSOAPowerConfigResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetSOAPowerConfigResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetSOATempConfigResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetSOATempConfigResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    // This command doesn't have a response from the signal generator.
                    Ok(_) => Response::SetUartBaudRate,
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::SetUartBaudRate(set_uart_baud_rate),
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::GetChannelID(get_channel_id),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                        let error_response = ReadWriteError::new(
                            Command::SetChannelID(set_channel_id),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetClockSourceResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetClockSourceResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetPowerMaxDbmResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetPowerMaxDbmResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetPowerMinDbmResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetPowerMinDbmResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<GetPowerOffsetResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<SetPowerOffsetResponse, _> =
                            sg_response.try_into();

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
                            e.kind,
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
                let command_response: Response = match write_read(&mut *self.port, command) {
                    Ok(sg_response) => {
                        let parse_result: Result<ResetSystemResponse, _> = sg_response.try_into();

                        match parse_result {
                            Ok(formatted_response) => {
                                Response::ResetSystemResponse(formatted_response)
                            }
                            Err(e) => Response::MWError(e),
                        }
                    }
                    // Return the command (for backtracking the source of issue) and the error description
                    Err(e) => {
                        let error_response = ReadWriteError::new(
                            Command::ResetSystem(reset_system),
                            e.kind,
                            e.description,
                        );

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
                let command_response: Response = match write_read(&mut *self.port, command) {
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
                            e.kind,
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

    /// This allows bundling commands to be executed in one process step.
    ///
    /// Ex. On update cycle, send any command that is still in the queue.
    pub fn handle_queue(&mut self) {
        let mut queue = Vec::new();
        while let Ok(msg) = self.queue.try_recv() {
            queue.push(msg);
        }

        // No commands to process.
        if queue.len() == 0 {
            return;
        };

        // Sort the queue commands by priority.
        queue.sort_by(|a, b| b.priority.cmp(&a.priority));

        for message in queue {
            let response = self.send(message.command);

            // Send the response to the channel so that it can be read by another process.
            let _ = self.channel.send(response);
        }
    }
}
