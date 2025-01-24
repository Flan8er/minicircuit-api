use crate::errors::errors::MWError;

pub enum Response {
    IdentityResponse,
    Error { value: MWError },
}

pub struct ClearErrorResponse {
    result: String,
}

pub struct SetChannelIDResponse {
    result: String,
}

pub struct ChannelIDResponse {
    command: String,
    channel: String,
}

pub struct SetClockSourceResponse {
    result: String,
}

pub struct GetClockSourceResponse {
    command: String,
    clock_source: ClockSource,
}

pub enum ClockSource {
    Standalone,
    Master,
    Slave,
    SlaveInline,
    Reserved,
}

impl ClockSource {
    pub fn new(source: u8) -> Self {
        match source {
            0 => Self::Standalone,
            1 => Self::Master,
            2 => Self::Slave,
            3 => Self::SlaveInline,
            _ => Self::Reserved,
        }
    }
}

pub struct SystemResetResponse {
    result: String,
}

pub struct ChangeRFOutputResponse {
    result: String,
}

pub struct GetRFStateResponse {
    command: String,
    enabled: bool,
}

pub struct SetFrequencyResponse {
    result: String,
}

pub struct GetFrequencyResponse {
    command: String,
    frequency: f32,
}

pub struct SetPhaseResponse {
    result: String,
}

pub struct GetPhaseResponse {
    command: String,
    degrees: f32,
}

pub struct GetPATempResponse {
    command: String,
    temp: f32,
}

pub struct SetVGAResponse {
    command: String,
}

pub struct GetVGAResponse {
    command: String,
    attenuation: f32,
}

pub struct SetMagnitudePercentResponse {
    command: String,
}

pub struct GetMagnitudePercentResponse {
    command: String,
    magnitude: u8,
}

pub struct SetISCPowerResponse {
    command: String,
}

pub struct GetPAForwardReflectedPowerWattResponse {
    command: String,
    forward_power: f32,
    reflected_power: f32,
}

pub struct GetPAForwardReflectedPowerDBMResponse {
    command: String,
    forward_power: f32,
    reflected_power: f32,
}

pub struct SetPAPowerSetpointWattResponse {
    command: String,
}

pub struct GetPAPowerSetpointWattResponse {
    command: String,
    power_watt: f32,
}

pub struct SetPAPowerSetpointDBMResponse {
    command: String,
}

pub struct GetPAPowerSetpointDBMResponse {
    command: String,
    power_watt: f32,
}

pub struct S11SweepWattResponse {
    command: String,
    frequency: f32,
    forward_power: f32,
    reflected_power: f32,
}

pub struct S11SweepDBMResponse {
    command: String,
    frequency: f32,
    forward_power: f32,
    reflected_power: f32,
}

pub struct SetDLLStateResponse {
    result: String,
}

pub struct GetDLLStateResponse {
    command: String,
    enabled: bool,
}

pub struct SetDLLConfigResponse {
    result: String,
}

pub struct GetDLLConfigResponse {
    command: String,
    lower_frequency: f32,
    upper_frequency: f32,
    start_frequency: f32,
    step_frequency: f32,
    threshold: f32,
    main_delay: u16,
}

pub struct SetPWMDutyCycleResponse {
    result: String,
}

pub struct SetSOAConfigResponse {
    result: String,
}

pub struct GetSOAConfigResponse {
    command: String,
    temp_enabled: bool,
    reflection_enabled: bool,
    external_watchdog_enabled: bool,
}

pub struct SetReflectedPowerSOAConfigResponse {
    result: String,
}

pub struct GetReflectedPowerSOAConfigResponse {
    command: String,
    high_reflection: f32,
    shutdown_reflection: f32,
}

pub struct SetTempSOAConfigResponse {
    result: String,
}

pub struct GetTempSOAConfigResponse {
    command: String,
    high_temp: f32,
    shutdown_temp: f32,
}

pub struct SetDissipationSOAConfigResponse {
    result: String,
}
