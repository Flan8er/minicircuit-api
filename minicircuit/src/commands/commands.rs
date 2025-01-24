use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    GetIdentity,
    GetVersion,
    GetStatus,
    ClearErrors,
    GetFrequency,
    SetFrequency(f32),
    GetPaPower,
    GetPowerSetpoint,
    SetPower(f32),
    ConfigureDll {
        param1: f32,
        param2: f32,
        param3: f32,
        param4: f32,
        param5: f32,
        param6: f32,
    },
    DllEnable,
    DllDisable,
    RfEnable,
    RfDisable,
    SweepDbm {
        start: f32,
        stop: f32,
        step: f32,
        power: f32,
    },
    GetTemperature,
}

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Command::GetIdentity => "$IDN,1".to_string(),
            Command::GetVersion => "$VER,1".to_string(),
            Command::GetStatus => "$ST,1".to_string(),
            Command::ClearErrors => "$ERRC,1".to_string(),
            Command::GetFrequency => "$FCG,1".to_string(),
            Command::SetFrequency(value) => format!("$FCS,1,{:.2}", value),
            Command::GetPaPower => "$PPG,1".to_string(),
            Command::GetPowerSetpoint => "$PWRG,1".to_string(),
            Command::SetPower(value) => format!("$PWRS,1,{:.2}", value),
            Command::ConfigureDll {
                param1,
                param2,
                param3,
                param4,
                param5,
                param6,
            } => format!(
                "$DLES,1,{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
                param1, param2, param3, param4, param5, param6
            ),
            Command::DllEnable => "$DLES,1,1".to_string(),
            Command::DllDisable => "$DLES,1,0".to_string(),
            Command::RfEnable => "$ECS,1,1".to_string(),
            Command::RfDisable => "$ECS,1,0".to_string(),
            Command::SweepDbm {
                start,
                stop,
                step,
                power,
            } => format!(
                "$SWPD,1,{:.2},{:.2},{:.2},{:.2},1",
                start, stop, step, power
            ),
            Command::GetTemperature => "$PTG,1".to_string(),
        }
    }
}
