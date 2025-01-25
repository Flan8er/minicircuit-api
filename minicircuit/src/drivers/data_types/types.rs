use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Frequency {
    pub frequency: u16,
}
impl Frequency {
    pub fn new(frequency: u16) -> Self {
        Self { frequency }
    }
}
impl Into<u16> for Frequency {
    fn into(self) -> u16 {
        self.frequency
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Channel {
    pub channel_id: u8,
}
impl Channel {
    pub fn new(channel_id: u8) -> Self {
        Self { channel_id }
    }
}
impl Default for Channel {
    fn default() -> Self {
        Self { channel_id: 1 }
    }
}
impl Into<u8> for Channel {
    fn into(self) -> u8 {
        self.channel_id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Watt {
    pub power: u8,
}
impl Watt {
    pub fn new(power: u8) -> Self {
        Self { power }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Dbm {
    pub power: u8,
}
impl Dbm {
    pub fn new(power: u8) -> Self {
        Self { power }
    }
}
impl Into<u8> for Dbm {
    fn into(self) -> u8 {
        self.power
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DutyCycle {
    pub duty_cycle: u8,
}
impl DutyCycle {
    pub fn new(duty_cycle: u8) -> Self {
        Self { duty_cycle }
    }
}
impl Default for DutyCycle {
    fn default() -> Self {
        Self { duty_cycle: 100 }
    }
}
impl Into<u8> for DutyCycle {
    fn into(self) -> u8 {
        self.duty_cycle
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Temperature {
    pub temperature: u8,
}
impl Temperature {
    pub fn new(temperature: u8) -> Self {
        Self { temperature }
    }
}
impl Into<u8> for Temperature {
    fn into(self) -> u8 {
        self.temperature
    }
}
