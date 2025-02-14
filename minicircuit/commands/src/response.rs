use crate::data_types::errors::{MWError, ReadWriteError};

use super::{
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

#[derive(Debug, Clone)]
/// The response can consist of feedback from the signal generator for the given command,
/// error from sending the command over serial connection, or error from the signal generator executing the command.
pub enum Response {
    GetPAPowerADCResponse(GetPAPowerADCResponse),
    GetPACurrentResponse(GetPACurrentResponse),
    GetPAPowerDBMResponse(GetPAPowerDBMResponse),
    GetPAPowerWattResponse(GetPAPowerWattResponse),
    GetFrequencyResponse(GetFrequencyResponse),
    SetFrequencyResponse(SetFrequencyResponse),
    GetRFOutputResponse(GetRFOutputResponse),
    SetRFOutputResponse(SetRFOutputResponse),
    GetPhaseResponse(GetPhaseResponse),
    SetPhaseResponse(SetPhaseResponse),
    GetPAPowerSetpointDBMResponse(GetPAPowerSetpointDBMResponse),
    GetPAPowerSetpointWattResponse(GetPAPowerSetpointWattResponse),
    SetPAPowerSetpointDBMResponse(SetPAPowerSetpointDBMResponse),
    SetPAPowerSetpointWattResponse(SetPAPowerSetpointWattResponse),
    GetPATempResponse(GetPATempResponse),
    GetPAVoltageResponse(GetPAVoltageResponse),
    GetDLLConfigResponse(GetDLLConfigResponse),
    SetDLLConfigResponse(SetDLLConfigResponse),
    GetDLLEnabledResponse(GetDLLEnabledResponse),
    SetDLLEnabledResponse(SetDLLEnabledResponse),
    PerformSweepDBMResponse(PerformSweepDBMResponse),
    PerformSweepWattResponse(PerformSweepWattResponse),
    ClearErrorsResponse(ClearErrorsResponse),
    GetPAErrorsResponse(GetPAErrorsResponse),
    GetStatusResponse(GetStatusResponse),
    GetIdentityResponse(GetIdentityResponse),
    GetISCTempResponse(GetISCTempResponse),
    GetUptimeResponse(GetUptimeResponse),
    GetVersionResponse(GetVersionResponse),
    GetAttenuationResponse(GetAttenuationResponse),
    SetAttenuationResponse(SetAttenuationResponse),
    GetAutoGainStateResponse(GetAutoGainStateResponse),
    SetAutoGainStateResponse(SetAutoGainStateResponse),
    GetMagnitudeResponse(GetMagnitudeResponse),
    SetMagnitudeResponse(SetMagnitudeResponse),
    GetISCPowerOutputResponse(GetISCPowerOutputResponse),
    SetISCPowerOutputResponse(SetISCPowerOutputResponse),
    GetPWMDutyCycleResponse(GetPWMDutyCycleResponse),
    SetPWMDutyCycleResponse(SetPWMDutyCycleResponse),
    SetPWMFrequencyResponse(SetPWMFrequencyResponse),
    SetTimedRFEnableResponse(SetTimedRFEnableResponse),
    GetSOAConfigResponse(GetSOAConfigResponse),
    SetSOAConfigResponse(SetSOAConfigResponse),
    GetSOACurrentConfigResponse(GetSOACurrentConfigResponse),
    SetSOACurrentConfigResponse(SetSOACurrentConfigResponse),
    GetSOADissipationConfigResponse(GetSOADissipationConfigResponse),
    SetSOADissipationConfigResponse(SetSOADissipationConfigResponse),
    GetSOAForwardPowerLimitsResponse(GetSOAForwardPowerLimitsResponse),
    SetSOAForwardPowerLimitsResponse(SetSOAForwardPowerLimitsResponse),
    SetSOAGraceTimerResponse(SetSOAGraceTimerResponse),
    GetSOAPowerConfigResponse(GetSOAPowerConfigResponse),
    SetSOAPowerConfigResponse(SetSOAPowerConfigResponse),
    GetSOATempConfigResponse(GetSOATempConfigResponse),
    SetSOATempConfigResponse(SetSOATempConfigResponse),
    GetSOAVoltageConfigResponse(GetSOAVoltageConfigResponse),
    SetSOAVoltageConfigResponse(SetSOAVoltageConfigResponse),
    SetSOAWatchdogConfigResponse(SetSOAWatchdogConfigResponse),
    GetChannelIDResponse(GetChannelIDResponse),
    SetChannelIDResponse(SetChannelIDResponse),
    GetClockSourceResponse(GetClockSourceResponse),
    SetClockSourceResponse(SetClockSourceResponse),
    SetCommunicationInterfaceResponse(SetCommunicationInterfaceResponse),
    GetPowerMaxDbmResponse(GetPowerMaxDbmResponse),
    SetPowerMaxDbmResponse(SetPowerMaxDbmResponse),
    GetPowerMinDbmResponse(GetPowerMinDbmResponse),
    SetPowerMinDbmResponse(SetPowerMinDbmResponse),
    GetPowerOffsetResponse(GetPowerOffsetResponse),
    SetPowerOffsetResponse(SetPowerOffsetResponse),
    ResetSystemResponse(ResetSystemResponse),
    SetZHLTriggerDelayResponse(SetZHLTriggerDelayResponse),
    ReadWriteError(ReadWriteError),
    MWError(MWError),
    SetUartBaudRate,
}

impl Into<String> for Response {
    fn into(self) -> String {
        let response = match self {
            Response::GetFrequencyResponse(get_frequency_response) => {
                format!(
                    "The frequency is currently {}MHz.",
                    get_frequency_response.frequency
                )
            }
            Response::SetFrequencyResponse(set_frequency_response) => {
                match set_frequency_response.result {
                    Ok(_) => format!("The frequency was sucessfully set."),
                    Err(e) => format!("An error occurred setting the frequency. \n{}", e),
                }
            }
            Response::ReadWriteError(read_write_error) => {
                format!(
                    "An error occurred sending a command to the signal generator. \n{}",
                    read_write_error.description
                )
            }
            Response::MWError(mwerror) => {
                format!("An error occurred executing a command. \n{}", mwerror)
            }
            Response::GetPAPowerADCResponse(get_papower_adcresponse) => {
                format!(
                    "The forward PA power ADC is currently {}ADC and reflected is {}ADC.",
                    get_papower_adcresponse.forward, get_papower_adcresponse.reflected
                )
            }
            Response::GetPACurrentResponse(get_pacurrent_response) => {
                format!(
                    "The current of the PA is currently {}A.",
                    get_pacurrent_response.current
                )
            }
            Response::GetPAPowerDBMResponse(get_papower_dbmresponse) => {
                format!(
                    "The PA forward power is currently {}dBm and reflected is {}dBm.",
                    get_papower_dbmresponse.forward, get_papower_dbmresponse.reflected
                )
            }
            Response::GetPAPowerWattResponse(get_papower_watt_response) => {
                format!(
                    "The PA forward power is currently {}W and reflected is {}W.",
                    get_papower_watt_response.forward, get_papower_watt_response.reflected
                )
            }
            Response::GetRFOutputResponse(get_rfoutput_response) => {
                let enabled_response = match get_rfoutput_response.enabled {
                    true => String::from("enabled"),
                    false => String::from("disabled"),
                };
                format!("The RF output is currently {}.", enabled_response)
            }
            Response::SetRFOutputResponse(set_rfoutput_response) => {
                match set_rfoutput_response.result {
                    Ok(_) => format!("The RF output mode was sucessfully set."),
                    Err(e) => format!("An error occurred setting the RF output mode. \n{}", e),
                }
            }
            Response::GetPhaseResponse(get_phase_response) => {
                format!(
                    "The ISC board's RF output phase is currently {}deg.",
                    get_phase_response.phase
                )
            }
            Response::SetPhaseResponse(set_phase_response) => match set_phase_response.result {
                Ok(_) => format!("The ISC board's RF output phase was sucessfully set."),
                Err(e) => format!(
                    "An error occurred setting the ISC board's RF output phase. \n{}",
                    e
                ),
            },
            Response::GetPAPowerSetpointDBMResponse(get_papower_setpoint_dbmresponse) => {
                format!(
                    "The PA output power setpoint is currently {}dBm.",
                    get_papower_setpoint_dbmresponse.power
                )
            }
            Response::GetPAPowerSetpointWattResponse(get_papower_setpoint_watt_response) => {
                format!(
                    "The PA output power setpoint is currently {}W.",
                    get_papower_setpoint_watt_response.power
                )
            }
            Response::SetPAPowerSetpointDBMResponse(set_papower_setpoint_dbmresponse) => {
                match set_papower_setpoint_dbmresponse.result {
                    Ok(_) => format!("The PA output power setpoint (dBm) was sucessfully set."),
                    Err(e) => format!(
                        "An error occurred setting the PA output power setpoint (dBm). \n{}",
                        e
                    ),
                }
            }
            Response::SetPAPowerSetpointWattResponse(set_papower_setpoint_watt_response) => {
                match set_papower_setpoint_watt_response.result {
                    Ok(_) => format!("The PA output power setpoint (W) was sucessfully set."),
                    Err(e) => format!(
                        "An error occurred setting the PA output power setpoint (W). \n{}",
                        e
                    ),
                }
            }
            Response::GetPATempResponse(get_patemp_response) => {
                format!(
                    "The PA temperature is currently {}degC.",
                    get_patemp_response.temperature
                )
            }
            Response::GetPAVoltageResponse(get_pavoltage_response) => {
                format!(
                    "The PA voltage is currently {}V.",
                    get_pavoltage_response.voltage
                )
            }
            Response::GetDLLConfigResponse(get_dllconfig_response) => {
                format!(
                    "The DLL configuration is currently: \nLower: {}MHz.\nUpper: {}MHz.\nStart: {}MHz.\nStep: {}MHz.\nThreshold: {}dB.\nMain Delay: {}ms.",
                    get_dllconfig_response.lower_frequency,
                    get_dllconfig_response.upper_frequency,
                    get_dllconfig_response.start_frequency,
                    get_dllconfig_response.step_frequency,
                    get_dllconfig_response.threshold,
                    get_dllconfig_response.main_delay,
                )
            }
            Response::SetDLLConfigResponse(set_dllconfig_response) => {
                match set_dllconfig_response.result {
                    Ok(_) => format!("The DLL configuration was sucessfully set."),
                    Err(e) => format!("An error occurred setting the DLL configuration. \n{}", e),
                }
            }
            Response::GetDLLEnabledResponse(get_dllenabled_response) => {
                let enabled_response = match get_dllenabled_response.enabled {
                    true => String::from("enabled"),
                    false => String::from("disabled"),
                };
                format!("The DLL mode is currently {}.", enabled_response)
            }
            Response::SetDLLEnabledResponse(set_dllenabled_response) => {
                match set_dllenabled_response.result {
                    Ok(_) => format!("The DLL mode was successfully set."),
                    Err(e) => format!("An error occurred setting the DLL mode. \n{}", e),
                }
            }
            Response::PerformSweepDBMResponse(perform_sweep_dbmresponse) => {
                format!(
                    "The most optimal frequency from the sweep is {}MHz. The following power readings were taken at that frequency:\nForward: {}dBm.\nReflected: {}dBm.",
                    perform_sweep_dbmresponse.measurement_frequency,
                    perform_sweep_dbmresponse.forward_power,
                    perform_sweep_dbmresponse.reflected_power
                )
            }
            Response::PerformSweepWattResponse(perform_sweep_watt_response) => {
                format!(
                    "The most optimal frequency from the sweep is {}MHz. The following power readings were taken at that frequency:\nForward: {}W.\nReflected: {}W.",
                    perform_sweep_watt_response.measurement_frequency,
                    perform_sweep_watt_response.forward_power,
                    perform_sweep_watt_response.reflected_power
                )
            }
            Response::ClearErrorsResponse(clear_errors_response) => {
                match clear_errors_response.result {
                    Ok(_) => format!("Errors on the ISC board were sucessfully cleared."),
                    Err(e) => format!(
                        "An error occurred clearing the errors on the ISC board. \n{}",
                        e
                    ),
                }
            }
            Response::GetPAErrorsResponse(get_paerrors_response) => {
                let mut combined_errors = String::new();
                for status in get_paerrors_response.pa_errors {
                    let status: String = status.into();

                    combined_errors.push_str(format!("\n{}", status).as_str());
                }

                format!("Error codes on the PA are: {}", combined_errors)
            }
            Response::GetStatusResponse(get_status_response) => {
                let mut combined_status = String::new();
                for status in get_status_response.status_codes {
                    let title = status.status;
                    let description = status.description;

                    combined_status.push_str(format!("\n{} {}", title, description).as_str());
                }

                format!("Status codes on the ISC board are: {}", combined_status)
            }
            Response::GetIdentityResponse(get_identity_response) => {
                format!(
                    "The ISC board identity is:\nModel: {}\nManufacturer: {}\nSerial Number: {}",
                    get_identity_response.isc_board,
                    get_identity_response.manufacturer,
                    get_identity_response.serial_number
                )
            }
            Response::GetISCTempResponse(get_isctemp_response) => {
                format!(
                    "The ISC board temperature is currently {}degC.",
                    get_isctemp_response.temperature
                )
            }
            Response::GetUptimeResponse(get_uptime_response) => {
                let whole_value: u64 = get_uptime_response.uptime.into();
                let minutes: f32 = whole_value as f32 / 60.;
                let hours: f32 = minutes / 60.;

                format!(
                    "The uptime of the ISC board is currently {:.1} hours.",
                    hours
                )
            }
            Response::GetVersionResponse(get_version_response) => {
                let version_response = match get_version_response.hotfix {
                    Some(hotfix) => {
                        format!("The ISC board version is: {}.{}.{}.\nHotfix: {}.\nManufacturer ID: {}.\nDate Stamp: {}.\nTime Stamp: {}.",
                            get_version_response.build,
                            get_version_response.major_version,
                            get_version_response.minor_version,
                            hotfix,
                            get_version_response.manufacturer_id,
                            get_version_response.date_stamp,
                            get_version_response.time_stamp
                        )
                    },
                    None => format!("The ISC board version is: {}.{}.{}.\nManufacturer ID: {}.\nDate Stamp: {}.\nTime Stamp: {}.",
                        get_version_response.build,
                        get_version_response.major_version,
                        get_version_response.minor_version,
                        get_version_response.manufacturer_id,
                        get_version_response.date_stamp,
                        get_version_response.time_stamp
                    )
                };
                version_response
            }
            Response::GetAttenuationResponse(get_attenuation_response) => {
                format!(
                    "The VGA attenuation is currently {}dB.",
                    get_attenuation_response.attenuation
                )
            }
            Response::SetAttenuationResponse(set_attenuation_response) => {
                match set_attenuation_response.result {
                    Ok(_) => format!("The VGA attenuation was sucessfully set."),
                    Err(e) => format!("An error occurred setting the VGA attenuation. \n{}", e),
                }
            }
            Response::GetAutoGainStateResponse(get_auto_gain_state_response) => {
                let auto_gain_response: String = match get_auto_gain_state_response.enabled {
                    true => String::from("enabled"),
                    false => String::from("disabled"),
                };
                format!("Auto-gain is currently {}.", auto_gain_response)
            }
            Response::SetAutoGainStateResponse(set_auto_gain_state_response) => {
                match set_auto_gain_state_response.result {
                    Ok(_) => format!("The auto-gain state was sucessfully set."),
                    Err(e) => format!("An error occurred setting the auto-gain state. \n{}", e),
                }
            }
            Response::GetMagnitudeResponse(get_magnitude_response) => {
                format!(
                    "The IQ modulator magnitude is currently {}%",
                    get_magnitude_response.magnitude
                )
            }
            Response::SetMagnitudeResponse(set_magnitude_response) => {
                match set_magnitude_response.result {
                    Ok(_) => format!("The IQ modulator magnitude was sucessfully set."),
                    Err(e) => format!("An error occurred setting the magnitude. \n{}", e),
                }
            }
            Response::GetISCPowerOutputResponse(get_iscpower_output_response) => {
                format!(
                    "The ISC Power output is currently {}dBm",
                    get_iscpower_output_response.power
                )
            }
            Response::SetISCPowerOutputResponse(set_iscpower_output_response) => {
                match set_iscpower_output_response.result {
                    Ok(_) => format!("The ISC power output was sucessfully set."),
                    Err(e) => format!("An error occurred setting the ISC power output. \n{}", e),
                }
            }
            Response::GetPWMDutyCycleResponse(get_pwmduty_cycle_response) => {
                format!(
                    "The PWM duty cycle is currently {}% at a frequency of {}Hz",
                    get_pwmduty_cycle_response.duty_cycle, get_pwmduty_cycle_response.frequency
                )
            }
            Response::SetPWMDutyCycleResponse(set_pwmduty_cycle_response) => {
                match set_pwmduty_cycle_response.result {
                    Ok(_) => format!("The PWM duty cycle was sucessfully set."),
                    Err(e) => format!("An error occurred setting the PWM duty cycle. \n{}", e),
                }
            }
            Response::SetPWMFrequencyResponse(set_pwmfrequency_response) => {
                match set_pwmfrequency_response.result {
                    Ok(_) => format!("The PWM frequency response was sucessfully set."),
                    Err(e) => format!(
                        "An error occurred setting the PWM frequency response. \n{}",
                        e
                    ),
                }
            }
            Response::SetTimedRFEnableResponse(set_timed_rfenable_response) => {
                match set_timed_rfenable_response.result {
                    Ok(_) => format!("The timed RF feature was sucessfully set."),
                    Err(e) => {
                        format!("An error occurred setting the timed RF feature. \n{}", e)
                    }
                }
            }
            Response::GetSOAConfigResponse(get_soaconfig_response) => {
                let watchdog_response: String =
                    match get_soaconfig_response.external_watchdog_enabled {
                        true => String::from("enabled"),
                        false => String::from("disabled"),
                    };
                let reflection_response: String = match get_soaconfig_response.reflection_enabled {
                    true => String::from("enabled"),
                    false => String::from("disabled"),
                };
                let temp_response: String = match get_soaconfig_response.temp_enabled {
                    true => String::from("enabled"),
                    false => String::from("disabled"),
                };
                format!(
                    "The SOA configuration is currently:\nExternal Watchdog: {}.\nReflection: {}.\nTemperature: {}.",
                    watchdog_response, reflection_response, temp_response
                )
            }
            Response::SetSOAConfigResponse(set_soaconfig_response) => {
                match set_soaconfig_response.result {
                    Ok(_) => format!("The SOA configuration was sucessfully set."),
                    Err(e) => {
                        format!("An error occurred setting the SOA configuration. \n{}", e)
                    }
                }
            }
            Response::GetSOACurrentConfigResponse(get_soacurrent_config_response) => {
                format!(
                    "The SOA current configuration is currently:\nHigh: {}A\nShutdown: {}A",
                    get_soacurrent_config_response.high_current,
                    get_soacurrent_config_response.shutdown_current
                )
            }
            Response::SetSOACurrentConfigResponse(set_soacurrent_config_response) => {
                match set_soacurrent_config_response.result {
                    Ok(_) => format!("The SOA current configuration was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the SOA current configuration. \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetSOADissipationConfigResponse(get_soadissipation_config_response) => {
                format!(
                    "The SOA dissipation configuration is currently:\nHigh: {}W\nShutdown: {}W",
                    get_soadissipation_config_response.high_dissipation,
                    get_soadissipation_config_response.shutdown_dissipation
                )
            }
            Response::SetSOADissipationConfigResponse(set_soadissipation_config_response) => {
                match set_soadissipation_config_response.result {
                    Ok(_) => format!("The SOA dissipation configuration was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the SOA dissipation configuration. \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetSOAForwardPowerLimitsResponse(get_soaforward_power_limits_response) => {
                format!(
                    "The SOA forward power limit configuration is currently:\nHigh: {}dBm\nShutdown: {}dBm",
                    get_soaforward_power_limits_response.high_forward_power,
                    get_soaforward_power_limits_response.shutdown_forward_power
                )
            }
            Response::SetSOAForwardPowerLimitsResponse(set_soaforward_power_limits_response) => {
                match set_soaforward_power_limits_response.result {
                    Ok(_) => {
                        format!("The SOA forward power limit configuration was sucessfully set.")
                    }
                    Err(e) => {
                        format!(
                            "An error occurred setting the SOA forward power limit configuration. \n{}",
                            e
                        )
                    }
                }
            }
            Response::SetSOAGraceTimerResponse(set_soagrace_timer_response) => {
                match set_soagrace_timer_response.result {
                    Ok(_) => format!("The SOA grace timer configuration was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the SOA grace timer configuration. \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetSOAPowerConfigResponse(get_soapower_config_response) => {
                format!(
                    "The SOA reflection power configuration is currently:\nHigh: {}dBm\nShutdown: {}dBm",
                    get_soapower_config_response.high_reflection,
                    get_soapower_config_response.shutdown_reflection
                )
            }
            Response::SetSOAPowerConfigResponse(set_soapower_config_response) => {
                match set_soapower_config_response.result {
                    Ok(_) => format!("The SOA power configuration was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the SOA power configuration. \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetSOATempConfigResponse(get_soatemp_config_response) => {
                format!(
                    "The SOA temperature configuration is currently:\nHigh: {}degC\nShutdown: {}degC",
                    get_soatemp_config_response.high_temp,
                    get_soatemp_config_response.shutdown_temp
                )
            }
            Response::SetSOATempConfigResponse(set_soatemp_config_response) => {
                match set_soatemp_config_response.result {
                    Ok(_) => format!("The SOA temperature configuration was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the SOA temperature configuration. \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetSOAVoltageConfigResponse(get_soavoltage_config_response) => {
                format!(
                    "The SOA voltage configuration is currently:\nHigh: {}V\nLow: {}V\nShutdown Minimum: {}V\nShutdown Maximum: {}V",
                    get_soavoltage_config_response.high_voltage,
                    get_soavoltage_config_response.low_voltage,
                    get_soavoltage_config_response.shutdown_min_voltage,
                    get_soavoltage_config_response.shutdown_max_voltage
                )
            }
            Response::SetSOAVoltageConfigResponse(set_soavoltage_config_response) => {
                match set_soavoltage_config_response.result {
                    Ok(_) => format!("The SOA voltage configuration was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the SOA voltage configuration. \n{}",
                            e
                        )
                    }
                }
            }
            Response::SetSOAWatchdogConfigResponse(set_soawatchdog_config_response) => {
                match set_soawatchdog_config_response.result {
                    Ok(_) => format!("The SOA watchdog configuration was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the SOA watchdog configuration. \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetChannelIDResponse(get_channel_idresponse) => {
                format!(
                    "The channel ID is currently {}.",
                    get_channel_idresponse.channel
                )
            }
            Response::SetChannelIDResponse(set_channel_idresponse) => {
                match set_channel_idresponse.result {
                    Ok(_) => format!("The channel ID was sucessfully set."),
                    Err(e) => {
                        format!("An error occurred setting the channel ID. \n{}", e)
                    }
                }
            }
            Response::GetClockSourceResponse(get_clock_source_response) => {
                let converted: String = get_clock_source_response.clock_source.into();
                format!("The clock source is currently \"{}\"", converted)
            }
            Response::SetClockSourceResponse(set_clock_source_response) => {
                match set_clock_source_response.result {
                    Ok(_) => format!("The clock source was sucessfully set."),
                    Err(e) => {
                        format!("An error occurred setting the clock source. \n{}", e)
                    }
                }
            }
            Response::SetCommunicationInterfaceResponse(set_communication_interface_response) => {
                match set_communication_interface_response.result {
                    Ok(_) => format!("The communication interface was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the communication interface. \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetPowerMaxDbmResponse(get_power_max_dbm_response) => {
                format!(
                    "The maximum output power is currently {}dBm",
                    get_power_max_dbm_response.max
                )
            }
            Response::SetPowerMaxDbmResponse(set_power_max_dbm_response) => {
                match set_power_max_dbm_response.result {
                    Ok(_) => format!("The maximum output power (dBm) was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the maximum output power (dBm). \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetPowerMinDbmResponse(get_power_min_dbm_response) => {
                format!(
                    "The minumum output power is currently {}dBm",
                    get_power_min_dbm_response.min
                )
            }
            Response::SetPowerMinDbmResponse(set_power_min_dbm_response) => {
                match set_power_min_dbm_response.result {
                    Ok(_) => format!("The minimum output power (dBm) was sucessfully set."),
                    Err(e) => {
                        format!(
                            "An error occurred setting the minimum output power (dBm). \n{}",
                            e
                        )
                    }
                }
            }
            Response::GetPowerOffsetResponse(get_power_offset_response) => {
                format!(
                    "The power offset is currently {}dB",
                    get_power_offset_response.offset
                )
            }
            Response::SetPowerOffsetResponse(set_power_offset_response) => {
                match set_power_offset_response.result {
                    Ok(_) => format!("Power offset was sucessfully set."),
                    Err(e) => {
                        format!("An error occurred setting the power offset. \n{}", e)
                    }
                }
            }
            Response::ResetSystemResponse(reset_system_response) => {
                match reset_system_response.result {
                    Ok(_) => format!("The system has sucessfully reset."),
                    Err(e) => {
                        format!("An error occurred resetting the system. \n{}", e)
                    }
                }
            }
            Response::SetZHLTriggerDelayResponse(set_zhltrigger_delay_response) => {
                match set_zhltrigger_delay_response.result {
                    Ok(_) => format!("The ZHL trigger delay was sucessfully set."),
                    Err(e) => {
                        format!("An error occurred setting the ZHL trigger delay. \n{}", e)
                    }
                }
            }
            Response::SetUartBaudRate => {
                format!("Updating UART baud rate command was successfully sent to the controller.")
            }
        };

        response
    }
}
