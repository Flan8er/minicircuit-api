use super::{
    auto_gain::{GetAutoGainState, SetAutoGainState},
    channel_id::{GetChannelID, SetChannelID},
    clear_errors::ClearErrors,
    clock_source::{GetClockSource, SetClockSource},
    dll_mode::{GetDLLConfig, GetDLLMode, SetDLLConfig, SetDLLMode},
    frequency::{GetFrequency, SetFrequency},
    identity::GetIdentity,
    iq_magnitude::{GetQIMagPercent, SetQIMagPercent},
    isc_power::{GetISCPowerOutput, SetISCPowerOutput},
    isc_temp::GetISCTemp,
    pa_power::{
        GetPACurrent, GetPAPowerADC, GetPAPowerDBM, GetPAPowerSetpointDBM, GetPAPowerSetpointWatt,
        GetPAPowerWatt, GetPAVoltage, SetPAPowerSetpointDBM, SetPAPowerSetpointWatt,
    },
    pa_temp::GetPATemp,
    phase::{GetPhase, SetPhase},
    pwm::{GetPWMSettings, SetPWMDutyCycle, SetTimedRFEnable},
    rf_output::{GetRFOutput, SetRFOutput},
    soa::{
        GetSOAConfig, GetSOACurrentConfig, GetSOADissipationConfig, GetSOAForwardPowerLimits,
        GetSOAPowerConfig, GetSOATempConfig, SetSOAConfig, SetSOACurrentConfig,
        SetSOADissipationConfig, SetSOAForwardPowerLimits, SetSOAPowerConfig, SetSOATempConfig,
    },
    status::GetStatus,
    sweep::{PerformSweepDBM, PerformSweepWatt},
    system_reset::ResetSystem,
    uptime::GetUptime,
    version::GetVersion,
    vga_attenuation::{GetVGAAttenuationDB, SetVGAAttenuationDB},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    GetIdentity(GetIdentity),
    GetUptime(GetUptime),
    GetVersion(GetVersion),
    GetStatus(GetStatus),
    ClearErrors(ClearErrors),
    SetChannelID(SetChannelID),
    GetChannelID(GetChannelID),
    SetClockSource(SetClockSource),
    GetClockSource(GetClockSource),
    ResetSystem(ResetSystem),
    GetRFOutput(GetRFOutput),
    SetRFOutput(SetRFOutput),
    GetFrequency(GetFrequency),
    SetFrequency(SetFrequency),
    GetPhase(GetPhase),
    SetPhase(SetPhase),
    GetPATemp(GetPATemp),
    GetVGAAttenuationDB(GetVGAAttenuationDB),
    SetVGAAttenuationDB(SetVGAAttenuationDB),
    GetQIMagPercent(GetQIMagPercent),
    SetQIMagPercent(SetQIMagPercent),
    SetISCPowerOutput(SetISCPowerOutput),
    GetISCPowerOutput(GetISCPowerOutput),
    GetISCTemp(GetISCTemp),
    GetPAPowerWatt(GetPAPowerWatt),
    GetPAPowerDBM(GetPAPowerDBM),
    GetPAPowerADC(GetPAPowerADC),
    GetPACurrent(GetPACurrent),
    GetPAVoltage(GetPAVoltage),
    SetPAPowerSetpointWatt(SetPAPowerSetpointWatt),
    GetPAPowerSetpointWatt(GetPAPowerSetpointWatt),
    SetPAPowerSetpointDBM(SetPAPowerSetpointDBM),
    GetPAPowerSetpointDBM(GetPAPowerSetpointDBM),
    PerformSweepWatt(PerformSweepWatt),
    PerformSweepDBM(PerformSweepDBM),
    GetDLLMode(GetDLLMode),
    SetDLLMode(SetDLLMode),
    GetDLLConfig(GetDLLConfig),
    SetDLLConfig(SetDLLConfig),
    GetPWMSettings(GetPWMSettings),
    SetPWMDutyCycle(SetPWMDutyCycle),
    SetTimedRFEnable(SetTimedRFEnable),
    SetSOAConfig(SetSOAConfig),
    SetSOATempConfig(SetSOATempConfig),
    SetSOAPowerConfig(SetSOAPowerConfig),
    GetSOAConfig(GetSOAConfig),
    GetSOATempConfig(GetSOATempConfig),
    GetSOAPowerConfig(GetSOAPowerConfig),
    GetSOACurrentConfig(GetSOACurrentConfig),
    SetSOACurrentConfig(SetSOACurrentConfig),
    GetSOADissipationConfig(GetSOADissipationConfig),
    SetSOADissipationConfig(SetSOADissipationConfig),
    GetSOAForwardPowerLimits(GetSOAForwardPowerLimits),
    SetSOAForwardPowerLimits(SetSOAForwardPowerLimits),
    SetAutoGainState(SetAutoGainState),
    GetAutoGainState(GetAutoGainState),
}
