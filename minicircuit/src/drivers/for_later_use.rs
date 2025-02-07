async fn test_command_responses() {
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
                        Err(e) => println!("An error occurred setting the frequency. \n{}", e),
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
                Response::GetPAPowerADCResponse(get_papower_adcresponse) => {
                    println!(
                        "PA Power ADC is set to: \nForward: {}\nReflected: {}",
                        get_papower_adcresponse.forward, get_papower_adcresponse.reflected
                    );
                }
                Response::GetPACurrentResponse(get_pacurrent_response) => {
                    println!("PA Current is set to: {}", get_pacurrent_response.current);
                }
                Response::GetPAPowerDBMResponse(get_papower_dbmresponse) => {
                    println!(
                        "PA Power DBM is set to: \nForward: {}\nReflected: {}",
                        get_papower_dbmresponse.forward, get_papower_dbmresponse.reflected
                    );
                }
                Response::GetPAPowerWattResponse(get_papower_watt_response) => {
                    println!(
                        "PA Power Watt is set to: \nForward: {}\nReflected: {}",
                        get_papower_watt_response.forward, get_papower_watt_response.reflected
                    );
                }
                Response::GetRFOutputResponse(get_rfoutput_response) => {
                    println!("RF output is enabled: {}", get_rfoutput_response.enabled);
                }
                Response::SetRFOutputResponse(set_rfoutput_response) => {
                    match set_rfoutput_response.result {
                        Ok(_) => println!("RF output was sucessfully set"),
                        Err(e) => println!("An error occurred enabling rf output. \n{}", e),
                    }
                }
                Response::GetPhaseResponse(get_phase_response) => {
                    println!("Phase is set to: {}", get_phase_response.phase);
                }
                Response::SetPhaseResponse(set_phase_response) => match set_phase_response.result {
                    Ok(_) => println!("Phase was sucessfully set"),
                    Err(e) => println!("An error occurred setting the phase. \n{}", e),
                },
                Response::GetPAPowerSetpointDBMResponse(get_papower_setpoint_dbmresponse) => {
                    println!(
                        "PA Power Setpoint DBM is set to: {}",
                        get_papower_setpoint_dbmresponse.power
                    );
                }
                Response::GetPAPowerSetpointWattResponse(get_papower_setpoint_watt_response) => {
                    println!(
                        "PA Power Setpoint Watt is set to: {}",
                        get_papower_setpoint_watt_response.power
                    );
                }
                Response::SetPAPowerSetpointDBMResponse(set_papower_setpoint_dbmresponse) => {
                    match set_papower_setpoint_dbmresponse.result {
                        Ok(_) => println!("PA Power setpoint dbm was sucessfully set."),
                        Err(e) => println!(
                            "An error occurred setting the pa power setpoitn dbm. \n{}",
                            e
                        ),
                    }
                }
                Response::SetPAPowerSetpointWattResponse(set_papower_setpoint_watt_response) => {
                    match set_papower_setpoint_watt_response.result {
                        Ok(_) => println!("PA Power setpoint watt was sucessfully set."),
                        Err(e) => println!(
                            "An error occurred setting the pa power setpoitn watt. \n{}",
                            e
                        ),
                    }
                }
                Response::GetPATempResponse(get_patemp_response) => {
                    println!("PA temp is currently: {}", get_patemp_response.temperature);
                }
                Response::GetPAVoltageResponse(get_pavoltage_response) => {
                    println!(
                        "PA Voltage is currently: {}",
                        get_pavoltage_response.voltage
                    );
                }
                Response::GetDLLConfigResponse(get_dllconfig_response) => {
                    println!(
                        "DLL Config is currently: \nLower: {}\nMain Delay: {}\nStart: {}\nStep: {}\nThreshold: {}\nUpper: {}",
                        get_dllconfig_response.lower_frequency,
                        get_dllconfig_response.main_delay,
                        get_dllconfig_response.start_frequency,
                        get_dllconfig_response.step_frequency,
                        get_dllconfig_response.threshold,
                        get_dllconfig_response.upper_frequency
                    );
                }
                Response::SetDLLConfigResponse(set_dllconfig_response) => {
                    match set_dllconfig_response.result {
                        Ok(_) => println!("DLL config was sucessfully set."),
                        Err(e) => println!("An error occurred setting the DLL config. \n{}", e),
                    }
                }
                Response::GetDLLEnabledResponse(get_dllenabled_response) => {
                    println!("DLL is enabled: {}", get_dllenabled_response.enabled);
                }
                Response::SetDLLEnabledResponse(set_dllenabled_response) => {
                    match set_dllenabled_response.result {
                        Ok(_) => println!("DLL was sucessfully enabled/disabled."),
                        Err(e) => println!("An error occurred setting the DLL enabled. \n{}", e),
                    }
                }
                Response::PerformSweepDBMResponse(perform_sweep_dbmresponse) => {
                    println!("Results from the DBM sweep are: \nForward: {}\nMeasured: {}\nReflected: {}",
                        perform_sweep_dbmresponse.forward_power,
                        perform_sweep_dbmresponse.measurement_frequency,
                        perform_sweep_dbmresponse.reflected_power
                    );
                }
                Response::PerformSweepWattResponse(perform_sweep_watt_response) => {
                    println!("Results from the Watt sweep are: \nForward: {}\nMeasured: {}\nReflected: {}",
                        perform_sweep_watt_response.forward_power,
                        perform_sweep_watt_response.measurement_frequency,
                        perform_sweep_watt_response.reflected_power
                    );
                }
                Response::ClearErrorsResponse(clear_errors_response) => {
                    match clear_errors_response.result {
                        Ok(_) => println!("Errors were sucessfully cleared."),
                        Err(e) => println!("An error occurred clearing the errors. \n{}", e),
                    }
                }
                Response::GetPAErrorsResponse(get_paerrors_response) => {
                    println!(
                        "Stored PA error code is: {}",
                        get_paerrors_response.pa_error_code
                    );
                }
                Response::GetStatusResponse(get_status_response) => {
                    println!(
                        "Status codes of the board are: {:#?}",
                        get_status_response.status_codes
                    )
                }
                Response::GetIdentityResponse(get_identity_response) => {
                    println!(
                        "Board identity is:\nISC Board: {}\nManufacturer: {}\nSerial Number: {}",
                        get_identity_response.isc_board,
                        get_identity_response.manufacturer,
                        get_identity_response.serial_number
                    );
                }
                Response::GetISCTempResponse(get_isctemp_response) => {
                    println!(
                        "ISC Temp is currently: {}",
                        get_isctemp_response.temperature
                    );
                }
                Response::GetUptimeResponse(get_uptime_response) => {
                    println!(
                        "Uptime is currently: {} seconds",
                        get_uptime_response.uptime
                    );
                }
                Response::GetVersionResponse(get_version_response) => {
                    println!("ISC board version is:\nBuild: {}\nDate Stamp: {}\nHotfix: {:#?}\nMajor Version: {}\nManufacturer ID: {}\nMinor Version: {}\nTime Stamp: {}",
                        get_version_response.build,
                        get_version_response.date_stamp,
                        get_version_response.hotfix,
                        get_version_response.major_version,
                        get_version_response.manufacturer_id,
                        get_version_response.minor_version,
                        get_version_response.time_stamp
                    );
                }
                Response::GetAttenuationResponse(get_attenuation_response) => {
                    println!(
                        "Attenuation is currently: {}",
                        get_attenuation_response.attenuation
                    );
                }
                Response::SetAttenuationResponse(set_attenuation_response) => {
                    match set_attenuation_response.result {
                        Ok(_) => println!("Attenuation was sucessfully set."),
                        Err(e) => println!("An error occurred setting the attenuation. \n{}", e),
                    }
                }
                Response::GetAutoGainStateResponse(get_auto_gain_state_response) => {
                    println!(
                        "Autogain state is currently enabled: {}",
                        get_auto_gain_state_response.enabled
                    );
                }
                Response::SetAutoGainStateResponse(set_auto_gain_state_response) => {
                    match set_auto_gain_state_response.result {
                        Ok(_) => println!("Autogain state was sucessfully set."),
                        Err(e) => println!("An error occurred setting the autogain state. \n{}", e),
                    }
                }
                Response::GetMagnitudeResponse(get_magnitude_response) => {
                    println!(
                        "Magnitude is currently set to: {}%",
                        get_magnitude_response.magnitude
                    );
                }
                Response::SetMagnitudeResponse(set_magnitude_response) => {
                    match set_magnitude_response.result {
                        Ok(_) => println!("Magnitude was sucessfully set."),
                        Err(e) => println!("An error occurred setting the magnitude. \n{}", e),
                    }
                }
                Response::GetISCPowerOutputResponse(get_iscpower_output_response) => {
                    println!(
                        "ISC Power output is currently: {}dBm",
                        get_iscpower_output_response.power
                    );
                }
                Response::SetISCPowerOutputResponse(set_iscpower_output_response) => {
                    match set_iscpower_output_response.result {
                        Ok(_) => println!("ISC power output  was sucessfully set."),
                        Err(e) => println!("An error occurred setting the magnitude. \n{}", e),
                    }
                }
                Response::GetPWMDutyCycleResponse(get_pwmduty_cycle_response) => {
                    println!(
                        "PWM Duty cycle is: \nDuty Cycle: {}%\nFrequency: {}",
                        get_pwmduty_cycle_response.duty_cycle, get_pwmduty_cycle_response.frequency
                    );
                }
                Response::SetPWMDutyCycleResponse(set_pwmduty_cycle_response) => {
                    match set_pwmduty_cycle_response.result {
                        Ok(_) => println!("PWM Duty Cycle was sucessfully set."),
                        Err(e) => println!("An error occurred setting the PWM Duty Cycle. \n{}", e),
                    }
                }
                Response::SetPWMFrequencyResponse(set_pwmfrequency_response) => {
                    match set_pwmfrequency_response.result {
                        Ok(_) => println!("PWM frequency response was sucessfully set."),
                        Err(e) => println!(
                            "An error occurred setting the PWM frequency response. \n{}",
                            e
                        ),
                    }
                }
                Response::SetTimedRFEnableResponse(set_timed_rfenable_response) => {
                    match set_timed_rfenable_response.result {
                        Ok(_) => println!("Timed RF enable was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the timed RF enable. \n{}", e)
                        }
                    }
                }
                Response::GetSOAConfigResponse(get_soaconfig_response) => {
                    println!(
                        "SOA config is currently:\nCurrent Enabled: {}\n
                        Dissipation Enabled: {}\n
                        External Watchdog Enabled: {}\n
                        IQ Modulator Enabled: {}\n
                        PA Status Enabled: {}\n
                        Reflection Enabled: {}\n
                        Temp Enabled: {}",
                        get_soaconfig_response.current_enabled,
                        get_soaconfig_response.dissipation_enabled,
                        get_soaconfig_response.external_watchdog_enabled,
                        get_soaconfig_response.iq_modulator_enabled,
                        get_soaconfig_response.pa_status_enabled,
                        get_soaconfig_response.reflection_enabled,
                        get_soaconfig_response.temp_enabled
                    );
                }
                Response::SetSOAConfigResponse(set_soaconfig_response) => {
                    match set_soaconfig_response.result {
                        Ok(_) => println!("SOA config was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the SOA config. \n{}", e)
                        }
                    }
                }
                Response::GetSOACurrentConfigResponse(get_soacurrent_config_response) => {
                    println!(
                        "SOA Current config is currently:\n
                        High Current: {}\n
                        Shutdown Current: {}",
                        get_soacurrent_config_response.high_current,
                        get_soacurrent_config_response.shutdown_current
                    );
                }
                Response::SetSOACurrentConfigResponse(set_soacurrent_config_response) => {
                    match set_soacurrent_config_response.result {
                        Ok(_) => println!("SOA current config was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the SOA current config. \n{}", e)
                        }
                    }
                }
                Response::GetSOADissipationConfigResponse(get_soadissipation_config_response) => {
                    println!(
                        "SOA Dissipation config is currently:\n
                        High Dissipation: {}\n
                        Shutdown Dissipation: {}",
                        get_soadissipation_config_response.high_dissipation,
                        get_soadissipation_config_response.shutdown_dissipation
                    )
                }
                Response::SetSOADissipationConfigResponse(set_soadissipation_config_response) => {
                    match set_soadissipation_config_response.result {
                        Ok(_) => println!("SOA dissipation config was sucessfully set."),
                        Err(e) => {
                            println!(
                                "An error occurred setting the SOA dissipation config. \n{}",
                                e
                            )
                        }
                    }
                }
                Response::GetSOAForwardPowerLimitsResponse(
                    get_soaforward_power_limits_response,
                ) => {
                    println!(
                        "SOA forward power limits are:\n
                        High Power: {}\n
                        Shutdown Power: {}",
                        get_soaforward_power_limits_response.high_forward_power,
                        get_soaforward_power_limits_response.shutdown_forward_power
                    );
                }
                Response::SetSOAForwardPowerLimitsResponse(
                    set_soaforward_power_limits_response,
                ) => match set_soaforward_power_limits_response.result {
                    Ok(_) => println!("SOA forward power config was sucessfully set."),
                    Err(e) => {
                        println!(
                            "An error occurred setting the SOA forward power config. \n{}",
                            e
                        )
                    }
                },
                Response::SetSOAGraceTimerResponse(set_soagrace_timer_response) => {
                    match set_soagrace_timer_response.result {
                        Ok(_) => println!("SOA grace timer config was sucessfully set."),
                        Err(e) => {
                            println!(
                                "An error occurred setting the SOA grace timer config. \n{}",
                                e
                            )
                        }
                    }
                }
                Response::GetSOAPowerConfigResponse(get_soapower_config_response) => {
                    println!(
                        "SOA power config is currently:\n
                        High Reflection: {}\n
                        Shutdown Reflection: {}",
                        get_soapower_config_response.high_reflection,
                        get_soapower_config_response.shutdown_reflection
                    );
                }
                Response::SetSOAPowerConfigResponse(set_soapower_config_response) => {
                    match set_soapower_config_response.result {
                        Ok(_) => println!("SOA power config was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the SOA power config. \n{}", e)
                        }
                    }
                }
                Response::GetSOATempConfigResponse(get_soatemp_config_response) => {
                    println!(
                        "SOA temp config is currently:\n
                        High Temp: {}\n
                        Shutdown Temp: {}",
                        get_soatemp_config_response.high_temp,
                        get_soatemp_config_response.shutdown_temp
                    );
                }
                Response::SetSOATempConfigResponse(set_soatemp_config_response) => {
                    match set_soatemp_config_response.result {
                        Ok(_) => println!("SOA temp config was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the SOA temp config. \n{}", e)
                        }
                    }
                }
                Response::GetSOAVoltageConfigResponse(get_soavoltage_config_response) => {
                    println!(
                        "SOA voltage config is currently:\n
                        High Voltage: {}\n
                        Low Voltage: {}\n
                        Shutdown Min Voltage: {}\n
                        Shutdown Max Voltage: {}",
                        get_soavoltage_config_response.high_voltage,
                        get_soavoltage_config_response.low_voltage,
                        get_soavoltage_config_response.shutdown_min_voltage,
                        get_soavoltage_config_response.shutdown_max_voltage
                    );
                }
                Response::SetSOAVoltageConfigResponse(set_soavoltage_config_response) => {
                    match set_soavoltage_config_response.result {
                        Ok(_) => println!("SOA voltage config was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the SOA voltage config. \n{}", e)
                        }
                    }
                }
                Response::SetSOAWatchdogConfigResponse(set_soawatchdog_config_response) => {
                    match set_soawatchdog_config_response.result {
                        Ok(_) => println!("SOA watchdog config was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the SOA watchdog config. \n{}", e)
                        }
                    }
                }
                Response::GetChannelIDResponse(get_channel_idresponse) => {
                    println!(
                        "Channel ID is currently: {}",
                        get_channel_idresponse.channel
                    );
                }
                Response::SetChannelIDResponse(set_channel_idresponse) => {
                    match set_channel_idresponse.result {
                        Ok(_) => println!("Channel ID was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the channel ID. \n{}", e)
                        }
                    }
                }
                Response::GetClockSourceResponse(get_clock_source_response) => {
                    let converted: u8 = get_clock_source_response.clock_source.into();
                    println!("Clocksource is currently set to: {}", converted);
                }
                Response::SetClockSourceResponse(set_clock_source_response) => {
                    match set_clock_source_response.result {
                        Ok(_) => println!("Clock source was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the clock source. \n{}", e)
                        }
                    }
                }
                Response::SetCommunicationInterfaceResponse(
                    set_communication_interface_response,
                ) => match set_communication_interface_response.result {
                    Ok(_) => println!("Communication interface was sucessfully set."),
                    Err(e) => {
                        println!(
                            "An error occurred setting the communication interface. \n{}",
                            e
                        )
                    }
                },
                Response::GetPowerMaxDbmResponse(get_power_max_dbm_response) => {
                    println!("Power max DBM is: {}dBm", get_power_max_dbm_response.max);
                }
                Response::SetPowerMaxDbmResponse(set_power_max_dbm_response) => {
                    match set_power_max_dbm_response.result {
                        Ok(_) => println!("Power max DbM was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the power max dBM. \n{}", e)
                        }
                    }
                }
                Response::GetPowerMinDbmResponse(get_power_min_dbm_response) => {
                    println!("Power min DBM is: {}dBm", get_power_min_dbm_response.min);
                }
                Response::SetPowerMinDbmResponse(set_power_min_dbm_response) => {
                    match set_power_min_dbm_response.result {
                        Ok(_) => println!("Power min DbM was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the power min dBM. \n{}", e)
                        }
                    }
                }
                Response::GetPowerOffsetResponse(get_power_offset_response) => {
                    println!(
                        "Power offset is currently: {}dB",
                        get_power_offset_response.offset
                    );
                }
                Response::SetPowerOffsetResponse(set_power_offset_response) => {
                    match set_power_offset_response.result {
                        Ok(_) => println!("Power offset was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the power offset. \n{}", e)
                        }
                    }
                }
                Response::ResetSystemResponse(reset_system_response) => {
                    match reset_system_response.result {
                        Ok(_) => println!("The system is/will sucessfully reset."),
                        Err(e) => {
                            println!("An error occurred resetting the system. \n{}", e)
                        }
                    }
                }
                Response::SetZHLTriggerDelayResponse(set_zhltrigger_delay_response) => {
                    match set_zhltrigger_delay_response.result {
                        Ok(_) => println!("The ZHL trigger delay was sucessfully set."),
                        Err(e) => {
                            println!("An error occurred setting the ZHL trigger delay. \n{}", e)
                        }
                    }
                }
                Response::SetUartBaudRate => {
                    println!("This command doesn't respond.");
                }
            };
        }
    });

    // A queue that can be used for sending commands to the driver.
    let (queue, queue_rx) = mpsc::channel::<Message>();

    // Connect to the signal generator that has the desired properties.
    let target_properties = TargetProperties::default();
    let port = match connect_to_signal_generator(target_properties) {
        Some(port) => port,
        None => {
            eprintln!("Exiting program: No valid connection.");
            return;
        }
    };

    let mut controller = MiniCircuitDriver::new(port, channel, queue_rx);

    // let command = Command::GetPAPowerADC(GetPAPowerADC::default());
    // let command = Command::GetPACurrent(GetPACurrent::default());
    // let command = Command::GetPAPowerDBM(GetPAPowerDBM::default());
    // let command = Command::GetPAPowerWatt(GetPAPowerWatt::default());
    // let command = Command::GetFrequency(GetFrequency::default());
    // let command = Command::SetFrequency(SetFrequency::default());
    // let command = Command::GetRFOutput(GetRFOutput::default());
    // let command = Command::SetRFOutput(SetRFOutput::default());
    // let command = Command::GetPhase(GetPhase::default());
    // let command = Command::SetPhase(SetPhase::default());
    // let command = Command::GetPAPowerSetpointDBM(GetPAPowerSetpointDBM::default());
    // let command = Command::GetPAPowerSetpointWatt(GetPAPowerSetpointWatt::default());
    // let command = Command::SetPAPowerSetpointDBM(SetPAPowerSetpointDBM::default());
    // let command = Command::SetPAPowerSetpointWatt(SetPAPowerSetpointWatt::default());
    // let command = Command::GetPATemp(GetPATemp::default());
    // let command = Command::GetPAVoltage(GetPAVoltage::default());
    // let command = Command::GetDLLConfig(GetDLLConfig::default());
    // let command = Command::SetDLLConfig(SetDLLConfig::default());
    // let command = Command::GetDLLEnabled(GetDLLEnabled::default());
    // let command = Command::SetDLLEnabled(SetDLLEnabled::default());
    // let command = Command::PerformSweepDBM(PerformSweepDBM::default());
    // let command = Command::PerformSweepWatt(PerformSweepWatt::default());
    // let command = Command::ClearErrors(ClearErrors::default());
    // let command = Command::GetPAErrors(GetPAErrors::default());
    // let command = Command::GetStatus(GetStatus::default());
    // let command = Command::GetIdentity(GetIdentity::default());
    // let command = Command::GetISCTemp(GetISCTemp::default());
    // let command = Command::GetUptime(GetUptime::default());
    // let command = Command::GetVersion(GetVersion::default());
    // let command = Command::GetAttenuation(GetAttenuation::default());
    // let command = Command::SetAttenuation(SetAttenuation::default());
    // let command = Command::GetAutoGainState(GetAutoGainState::default());
    // let command = Command::SetAutoGainState(SetAutoGainState::new(Channel::default(), false));
    // let command = Command::GetMagnitude(GetMagnitude::default());
    // let command = Command::SetMagnitude(SetMagnitude::default());
    // let command = Command::GetISCPowerOutput(GetISCPowerOutput::default());
    // let command = Command::SetISCPowerOutput(SetISCPowerOutput::default());
    // let command = Command::GetPWMDutyCycle(GetPWMDutyCycle::default());
    // let command = Command::SetPWMDutyCycle(SetPWMDutyCycle::default());
    // let command = Command::SetPWMFrequency(SetPWMFrequency::default());
    // let command = Command::SetTimedRFEnable(SetTimedRFEnable::default());
    // let command = Command::GetSOAConfig(GetSOAConfig::default());
    // let command = Command::SetSOAConfig(SetSOAConfig::new(
    //     Channel::new(1),
    //     SOAType::Dissipation,
    //     true,
    // ));
    // let command = Command::GetSOACurrentConfig(GetSOACurrentConfig::default());
    // let command = Command::SetSOACurrentConfig(SetSOACurrentConfig::default());
    // let command = Command::GetSOADissipationConfig(GetSOADissipationConfig::default());
    // let command = Command::SetSOADissipationConfig(SetSOADissipationConfig::default());
    // let command = Command::GetSOAForwardPowerLimits(GetSOAForwardPowerLimits::default());
    // let command = Command::SetSOAForwardPowerLimits(SetSOAForwardPowerLimits::default());
    // let command = Command::SetSOAGraceTimer(SetSOAGraceTimer::default());
    // let command = Command::GetSOAPowerConfig(GetSOAPowerConfig::default());
    // let command = Command::SetSOAPowerConfig(SetSOAPowerConfig::default());
    // let command = Command::GetSOATempConfig(GetSOATempConfig::default());
    // let command = Command::SetSOATempConfig(SetSOATempConfig::default());
    // let command = Command::GetSOAVoltageConfig(GetSOAVoltageConfig::default());
    // let command = Command::SetSOAVoltageConfig(SetSOAVoltageConfig::default());
    // let command = Command::SetSOAWatchdogConfig(SetSOAWatchdogConfig::default());
    // let command = Command::SetUartBaudRate(SetUartBaudRate::default());
    // let command = Command::GetChannelID(GetChannelID::default());
    // let command = Command::SetChannelID(SetChannelID::new(Channel::new(1), Channel::new(1)));
    // let command = Command::GetClockSource(GetClockSource::default());
    // let command = Command::SetClockSource(SetClockSource::default());
    // let command = Command::SetCommunicationInterface(SetCommunicationInterface::default());
    // let command = Command::GetPowerMaxDbm(GetPowerMaxDbm::default());
    // let command = Command::SetPowerMaxDbm(SetPowerMaxDbm::default());
    // let command = Command::GetPowerMinDbm(GetPowerMinDbm::default());
    // let command = Command::SetPowerMinDbm(SetPowerMinDbm::default());
    // let command = Command::GetPowerOffset(GetPowerOffset::default());
    // let command = Command::SetPowerOffset(SetPowerOffset::default());
    // let command = Command::ResetSystem(ResetSystem::default());
    // let command = Command::SetZHLTriggerDelay(SetZHLTriggerDelay::default());

    // Giving the "setter" function higher priority so that it is executed before the "getter".
    // This ensures the getter is returning the current state.
    let _ = queue.send(Message {
        priority: Priority::Low,
        command: command.clone(),
    });

    // Telling the driver to execute all the commands that are in it's queue.
    controller.handle_queue();

    // Parse the responses from the queue as a result of executing the commands.
    handle.await.unwrap();
}
