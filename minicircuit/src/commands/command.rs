use serde::{Deserialize, Serialize};

use crate::commands::{
    basic::{
        adc::GetPAPowerADC,
        current::GetPACurrent,
        forward_reflected::{GetPAPowerDBM, GetPAPowerWatt},
        frequency::{GetFrequency, SetFrequency},
        output::{GetRFOutput, SetRFOutput},
        phase::{GetPhase, SetPhase},
        setpoint::{
            GetPAPowerSetpointDBM, GetPAPowerSetpointWatt, SetPAPowerSetpointDBM,
            SetPAPowerSetpointWatt,
        },
        temperature::GetPATemp,
        voltage::GetPAVoltage,
    },
    dll::{
        config::{GetDLLConfig, SetDLLConfig},
        enable::{GetDLLEnabled, SetDLLEnabled},
        sweep::{PerformSweepDBM, PerformSweepWatt},
    },
    error::{clear_errors::ClearErrors, pa::GetPAErrors, status::GetStatus},
    information::{
        identity::GetIdentity, isc_temp::GetISCTemp, uptime::GetUptime, version::GetVersion,
    },
    manual::{
        attenuation::{GetAttenuation, SetAttenuation},
        auto_gain::{GetAutoGainState, SetAutoGainState},
        magnitude::{GetMagnitude, SetMagnitude},
        power::{GetISCPowerOutput, SetISCPowerOutput},
    },
    pwm::{
        duty_cycle::{GetPWMDutyCycle, SetPWMDutyCycle},
        frequency::SetPWMFrequency,
        timed_rf::SetTimedRFEnable,
    },
    soa::{
        config::{GetSOAConfig, SetSOAConfig},
        current::{GetSOACurrentConfig, SetSOACurrentConfig},
        dissipation::{GetSOADissipationConfig, SetSOADissipationConfig},
        forward_power::{GetSOAForwardPowerLimits, SetSOAForwardPowerLimits},
        grace_timer::SetSOAGraceTimer,
        reflected_power::{GetSOAPowerConfig, SetSOAPowerConfig},
        temperature::{GetSOATempConfig, SetSOATempConfig},
        voltage::{GetSOAVoltageConfig, SetSOAVoltageConfig},
        watchdog::SetSOAWatchdogConfig,
    },
    system::{
        baud_rate::SetUartBaudRate,
        channel_id::{GetChannelID, SetChannelID},
        clock_source::{GetClockSource, SetClockSource},
        communication::SetCommunicationInterface,
        power_max::{GetPowerMaxDbm, SetPowerMaxDbm},
        power_min::{GetPowerMinDbm, SetPowerMinDbm},
        power_offset::{GetPowerOffset, SetPowerOffset},
        system_reset::ResetSystem,
        trigger_delay::SetZHLTriggerDelay,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    GetPAPowerADC(GetPAPowerADC),
    GetPACurrent(GetPACurrent),
    GetPAPowerDBM(GetPAPowerDBM),
    GetPAPowerWatt(GetPAPowerWatt),
    GetFrequency(GetFrequency),
    SetFrequency(SetFrequency),
    GetRFOutput(GetRFOutput),
    SetRFOutput(SetRFOutput),
    GetPhase(GetPhase),
    SetPhase(SetPhase),
    GetPAPowerSetpointDBM(GetPAPowerSetpointDBM),
    GetPAPowerSetpointWatt(GetPAPowerSetpointWatt),
    SetPAPowerSetpointDBM(SetPAPowerSetpointDBM),
    SetPAPowerSetpointWatt(SetPAPowerSetpointWatt),
    GetPATemp(GetPATemp),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPAVoltage(GetPAVoltage),
    GetDLLConfig(GetDLLConfig),
    SetDLLConfig(SetDLLConfig),
    GetDLLEnabled(GetDLLEnabled),
    SetDLLEnabled(SetDLLEnabled),
    PerformSweepDBM(PerformSweepDBM),
    PerformSweepWatt(PerformSweepWatt),
    ClearErrors(ClearErrors),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPAErrors(GetPAErrors),
    GetStatus(GetStatus),
    GetIdentity(GetIdentity),
    GetISCTemp(GetISCTemp),
    GetUptime(GetUptime),
    GetVersion(GetVersion),
    GetAttenuation(GetAttenuation),
    SetAttenuation(SetAttenuation),
    GetAutoGainState(GetAutoGainState),
    SetAutoGainState(SetAutoGainState),
    GetMagnitude(GetMagnitude),
    SetMagnitude(SetMagnitude),
    GetISCPowerOutput(GetISCPowerOutput),
    SetISCPowerOutput(SetISCPowerOutput),
    GetPWMDutyCycle(GetPWMDutyCycle),
    SetPWMDutyCycle(SetPWMDutyCycle),
    SetPWMFrequency(SetPWMFrequency),
    SetTimedRFEnable(SetTimedRFEnable),
    GetSOAConfig(GetSOAConfig),
    SetSOAConfig(SetSOAConfig),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetSOACurrentConfig(GetSOACurrentConfig),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOACurrentConfig(SetSOACurrentConfig),
    GetSOADissipationConfig(GetSOADissipationConfig),
    SetSOADissipationConfig(SetSOADissipationConfig),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetSOAForwardPowerLimits(GetSOAForwardPowerLimits),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOAForwardPowerLimits(SetSOAForwardPowerLimits),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOAGraceTimer(SetSOAGraceTimer),
    GetSOAPowerConfig(GetSOAPowerConfig),
    SetSOAPowerConfig(SetSOAPowerConfig),
    GetSOATempConfig(GetSOATempConfig),
    SetSOATempConfig(SetSOATempConfig),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetSOAVoltageConfig(GetSOAVoltageConfig),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOAVoltageConfig(SetSOAVoltageConfig),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOAWatchdogConfig(SetSOAWatchdogConfig),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetUartBaudRate(SetUartBaudRate),
    GetChannelID(GetChannelID),
    SetChannelID(SetChannelID),
    GetClockSource(GetClockSource),
    SetClockSource(SetClockSource),
    SetCommunicationInterface(SetCommunicationInterface),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPowerMaxDbm(GetPowerMaxDbm),
    SetPowerMaxDbm(SetPowerMaxDbm),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPowerMinDbm(GetPowerMinDbm),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetPowerMinDbm(SetPowerMinDbm),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPowerOffset(GetPowerOffset),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetPowerOffset(SetPowerOffset),
    ResetSystem(ResetSystem),
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetZHLTriggerDelay(SetZHLTriggerDelay),
}
