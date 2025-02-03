use serde::{Deserialize, Serialize};
use serialport::{Error, SerialPort};

use crate::commands::commands::Command;

use super::communication::write_read;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    pub channel: tokio::sync::broadcast::Sender<String>,
    // queue: tokio::sync::mpsc::Sender<Message>,
}

impl MiniCircuitDriver {
    pub fn send(&mut self, command: Command) -> Result<String, Error> {
        let command: String = match command {
            Command::GetPAPowerADC(get_papower_adc) => get_papower_adc.into(),
            Command::GetPACurrent(get_pacurrent) => get_pacurrent.into(),
            Command::GetPAPowerDBM(get_papower_dbm) => get_papower_dbm.into(),
            Command::GetPAPowerWatt(get_papower_watt) => get_papower_watt.into(),
            Command::GetFrequency(get_frequency) => get_frequency.into(),
            Command::SetFrequency(set_frequency) => set_frequency.into(),
            Command::GetRFOutput(get_rfoutput) => get_rfoutput.into(),
            Command::SetRFOutput(set_rfoutput) => set_rfoutput.into(),
            Command::GetPhase(get_phase) => get_phase.into(),
            Command::SetPhase(set_phase) => set_phase.into(),
            Command::GetPAPowerSetpointDBM(get_papower_setpoint_dbm) => {
                get_papower_setpoint_dbm.into()
            }
            Command::GetPAPowerSetpointWatt(get_papower_setpoint_watt) => {
                get_papower_setpoint_watt.into()
            }
            Command::SetPAPowerSetpointDBM(set_papower_setpoint_dbm) => {
                set_papower_setpoint_dbm.into()
            }
            Command::SetPAPowerSetpointWatt(set_papower_setpoint_watt) => {
                set_papower_setpoint_watt.into()
            }
            Command::GetPATemp(get_patemp) => get_patemp.into(),
            Command::GetPAVoltage(get_pavoltage) => get_pavoltage.into(),
            Command::GetDLLConfig(get_dllconfig) => get_dllconfig.into(),
            Command::SetDLLConfig(set_dllconfig) => set_dllconfig.into(),
            Command::GetDLLEnabled(get_dllenabled) => get_dllenabled.into(),
            Command::SetDLLEnabled(set_dllenabled) => set_dllenabled.into(),
            Command::PerformSweepDBM(perform_sweep_dbm) => perform_sweep_dbm.into(),
            Command::PerformSweepWatt(perform_sweep_watt) => perform_sweep_watt.into(),
            Command::ClearErrors(clear_errors) => clear_errors.into(),
            Command::GetPAErrors(get_paerrors) => get_paerrors.into(),
            Command::GetStatus(get_status) => get_status.into(),
            Command::GetIdentity(get_identity) => get_identity.into(),
            Command::GetISCTemp(get_isctemp) => get_isctemp.into(),
            Command::GetUptime(get_uptime) => get_uptime.into(),
            Command::GetVersion(get_version) => get_version.into(),
            Command::GetAttenuation(get_attenuation) => get_attenuation.into(),
            Command::SetAttenuation(set_attenuation) => set_attenuation.into(),
            Command::GetAutoGainState(get_auto_gain_state) => get_auto_gain_state.into(),
            Command::SetAutoGainState(set_auto_gain_state) => set_auto_gain_state.into(),
            Command::GetMagnitude(get_magnitude) => get_magnitude.into(),
            Command::SetMagnitude(set_magnitude) => set_magnitude.into(),
            Command::GetISCPowerOutput(get_iscpower_output) => get_iscpower_output.into(),
            Command::SetISCPowerOutput(set_iscpower_output) => set_iscpower_output.into(),
            Command::GetPWMDutyCycle(get_pwmduty_cycle) => get_pwmduty_cycle.into(),
            Command::SetPWMDutyCycle(set_pwmduty_cycle) => set_pwmduty_cycle.into(),
            Command::SetPWMFrequency(set_pwmfrequency) => set_pwmfrequency.into(),
            Command::SetTimedRFEnable(set_timed_rfenable) => set_timed_rfenable.into(),
            Command::GetSOAConfig(get_soaconfig) => get_soaconfig.into(),
            Command::SetSOAConfig(set_soaconfig) => set_soaconfig.into(),
            Command::GetSOACurrentConfig(get_soacurrent_config) => get_soacurrent_config.into(),
            Command::SetSOACurrentConfig(set_soacurrent_config) => set_soacurrent_config.into(),
            Command::GetSOADissipationConfig(get_soadissipation_config) => {
                get_soadissipation_config.into()
            }
            Command::SetSOADissipationConfig(set_soadissipation_config) => {
                set_soadissipation_config.into()
            }
            Command::GetSOAForwardPowerLimits(get_soaforward_power_limits) => {
                get_soaforward_power_limits.into()
            }
            Command::SetSOAForwardPowerLimits(set_soaforward_power_limits) => {
                set_soaforward_power_limits.into()
            }
            Command::SetSOAGraceTimer(set_soagrace_timer) => set_soagrace_timer.into(),
            Command::GetSOAPowerConfig(get_soapower_config) => get_soapower_config.into(),
            Command::SetSOAPowerConfig(set_soapower_config) => set_soapower_config.into(),
            Command::GetSOATempConfig(get_soatemp_config) => get_soatemp_config.into(),
            Command::SetSOATempConfig(set_soatemp_config) => set_soatemp_config.into(),
            Command::GetSOAVoltageConfig(get_soavoltage_config) => get_soavoltage_config.into(),
            Command::SetSOAVoltageConfig(set_soavoltage_config) => set_soavoltage_config.into(),
            Command::SetSOAWatchdogConfig(set_soawatchdog_config) => set_soawatchdog_config.into(),
            Command::SetUartBaudRate(set_uart_baud_rate) => set_uart_baud_rate.into(),
            Command::GetChannelID(get_channel_id) => get_channel_id.into(),
            Command::SetChannelID(set_channel_id) => set_channel_id.into(),
            Command::GetClockSource(get_clock_source) => get_clock_source.into(),
            Command::SetClockSource(set_clock_source) => set_clock_source.into(),
            Command::SetCommunicationInterface(set_communication_interface) => {
                set_communication_interface.into()
            }
            Command::GetPowerMaxDbm(get_power_max_dbm) => get_power_max_dbm.into(),
            Command::SetPowerMaxDbm(set_power_max_dbm) => set_power_max_dbm.into(),
            Command::GetPowerMinDbm(get_power_min_dbm) => get_power_min_dbm.into(),
            Command::SetPowerMinDbm(set_power_min_dbm) => set_power_min_dbm.into(),
            Command::GetPowerOffset(get_power_offset) => get_power_offset.into(),
            Command::SetPowerOffset(set_power_offset) => set_power_offset.into(),
            Command::ResetSystem(reset_system) => reset_system.into(),
            Command::SetZHLTriggerDelay(set_zhltrigger_delay) => set_zhltrigger_delay.into(),
        };

        write_read(&mut *self.port, command)
    }
}
