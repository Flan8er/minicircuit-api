use serde::{Deserialize, Serialize};
use serialport::{Error, SerialPort};

use crate::commands::{basic::adc::GetPAPowerADCResponse, command::Command, response::Response};

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
            Command::GetPACurrent(get_pacurrent) => todo!(),
            Command::GetPAPowerDBM(get_papower_dbm) => todo!(),
            Command::GetPAPowerWatt(get_papower_watt) => todo!(),
            Command::GetFrequency(get_frequency) => todo!(),
            Command::SetFrequency(set_frequency) => todo!(),
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
        }
    }

    /// This allows bundling commands to be executed in one process step.
    ///
    /// Ex. On update cycle, send any command that is still in the queue.
    pub fn handle_queue(&mut self) {
        let mut queue: Vec<Message> = self.queue.iter().collect();
        // Sort the queue commands by priority.
        queue.sort_by(|a, b| b.priority.cmp(&a.priority));

        for message in queue {
            match message.command {
                Command::GetPAPowerADC(get_papower_adc) => {
                    // Convert the command into a string (required format to be sent to the signal generator).
                    let command: String = get_papower_adc.clone().into();

                    // Collect the resulting response of sending the command.
                    let command_response: Response = match write_read(&mut *self.port, command) {
                        Ok(sg_response) => {
                            let parse_result: Result<GetPAPowerADCResponse, _> =
                                sg_response.try_into();

                            match parse_result {
                                Ok(formatted_response) => {
                                    Response::GetPAPowerADCResponse(formatted_response)
                                }
                                Err(e) => Response::MWError(e),
                            }
                        }
                        Err(e) => {
                            let error_response = ReadWriteError::new(
                                Command::GetPAPowerADC(get_papower_adc),
                                e.kind,
                                e.description,
                            );

                            Response::ReadWriteError(error_response)
                        }
                    };

                    // Send the response to the channel so that it can be read by another process.
                    let _ = self.channel.send(command_response);
                }
                Command::GetPACurrent(get_pacurrent) => todo!(),
                Command::GetPAPowerDBM(get_papower_dbm) => todo!(),
                Command::GetPAPowerWatt(get_papower_watt) => todo!(),
                Command::GetFrequency(get_frequency) => todo!(),
                Command::SetFrequency(set_frequency) => todo!(),
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
        }
    }
}
