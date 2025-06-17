use log::{debug, info};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use minicircuit_commands::data_types::types::Channel;

/// Simulates a MiniCircuit device by processing commands and generating responses
pub struct MiniCircuitSimulator {
    // Store device state
    frequency: f64,
    rf_output_enabled: bool,
    phase: f64,
    channel_id: Channel,
    power_dbm: f64,
    power_watt: f64,
    attenuation: f64,
    magnitude: f64,
    temperature: f64,
    voltage: f64,
    current: f64,
    start_time: Instant,
    // Add more state variables as needed
    command_log: Vec<String>,
}

impl MiniCircuitSimulator {
    pub fn new() -> Self {
        Self {
            frequency: 2400.0, // Default frequency in MHz
            rf_output_enabled: false,
            phase: 0.0,
            channel_id: Channel::default(),
            power_dbm: 10.0,
            power_watt: 0.01,
            attenuation: 20.0,
            magnitude: 0.5,
            temperature: 35.5,
            voltage: 12.0,
            current: 0.5,
            start_time: Instant::now(),
            command_log: Vec::new(),
        }
    }

    // Add a method to get the command log
    pub fn get_command_log(&self) -> &Vec<String> {
        &self.command_log
    }

    /// Process a command string and return the appropriate response
    pub fn process_command(&mut self, command: &str) -> String {
        let command = command.trim();
        info!("Processing command: {}", command);
        
        // Log the command
        self.command_log.push(command.to_string());
        
        // Parse the command
        let parts: Vec<&str> = command.split(',').collect();
        if parts.is_empty() {
            return "ERROR: Empty command".to_string();
        }
        
        // Process the command and return the response
        let response = match parts[0] {
            // Basic frequency commands
            "$FCG" => self.handle_get_frequency(),
            "$FCS" => self.handle_set_frequency(parts),
            
            // RF output commands
            "$ECS" => self.handle_set_rf_output(parts),
            "$ECG" => self.handle_get_rf_output(),
            
            // Phase commands
            "$PCG" => self.handle_get_phase(),
            "$PCS" => self.handle_set_phase(parts),
            
            // Identity and information commands
            "$IDN" => self.handle_get_identity(),
            "$TCG" => self.handle_get_isc_temp(),
            "$RTG" => self.handle_get_uptime(),
            "$ST" => self.handle_get_status(),
            "$RST" => self.handle_reset_system(),
            
            // Power commands
            "$PFG" => self.handle_get_power_dbm(),
            "$PFS" => self.handle_set_power_dbm(parts),
            "$PWG" => self.handle_get_power_watt(),
            "$PWS" => self.handle_set_power_watt(parts),
            
            // Attenuation and magnitude commands
            "$ATG" => self.handle_get_attenuation(),
            "$ATS" => self.handle_set_attenuation(parts),
            "$MAG" => self.handle_get_magnitude(),
            "$MAS" => self.handle_set_magnitude(parts),
            
            // Temperature, voltage, and current commands
            "$TPG" => self.handle_get_pa_temp(),
            "$VTG" => self.handle_get_pa_voltage(),
            "$CTG" => self.handle_get_pa_current(),
            
            // Channel ID commands
            "$CIG" => self.handle_get_channel_id(),
            "$CIS" => self.handle_set_channel_id(parts),
            
            // Add more command handlers as needed
            _ => format!("ERROR: Unknown command {}", parts[0]),
        };
        
        info!("Command response: {}", response);
        response
    }

    fn handle_get_frequency(&self) -> String {
        format!("OK,{},{:.2}", self.channel_id.channel_id, self.frequency)
    }

    fn handle_set_frequency(&mut self, parts: Vec<&str>) -> String {
        if parts.len() < 3 {
            return "ERROR: Invalid command format".to_string();
        }
        
        match parts[2].parse::<f64>() {
            Ok(freq) => {
                self.frequency = freq;
                "OK".to_string()
            },
            Err(_) => "ERROR: Invalid frequency value".to_string(),
        }
    }

    fn handle_set_rf_output(&mut self, parts: Vec<&str>) -> String {
        if parts.len() < 3 {
            return "ERROR: Invalid command format".to_string();
        }
        
        match parts[2].parse::<u8>() {
            Ok(value) => {
                self.rf_output_enabled = value == 1;
                "OK".to_string()
            },
            Err(_) => "ERROR: Invalid value".to_string(),
        }
    }

    fn handle_get_rf_output(&self) -> String {
        format!("OK,{},{}", self.channel_id.channel_id, if self.rf_output_enabled { 1 } else { 0 })
    }

    fn handle_get_phase(&self) -> String {
        format!("OK,{},{:.2}", self.channel_id.channel_id, self.phase)
    }

    fn handle_set_phase(&mut self, parts: Vec<&str>) -> String {
        if parts.len() < 3 {
            return "ERROR: Invalid command format".to_string();
        }
        
        match parts[2].parse::<f64>() {
            Ok(phase) => {
                self.phase = phase;
                "OK".to_string()
            },
            Err(_) => "ERROR: Invalid phase value".to_string(),
        }
    }

    fn handle_get_identity(&self) -> String {
        // Format the identity response exactly as expected by the parser
        // The format should be: "OK,<channel>,<manufacturer> <isc_board>,<serial_number>"
        format!("OK,{},MiniCircuits ISC-2400-XX,SN12345678", self.channel_id.channel_id)
    }

    fn handle_get_isc_temp(&self) -> String {
        format!("OK,{},{}", self.channel_id.channel_id, self.temperature)
    }

    fn handle_get_uptime(&self) -> String {
        let uptime = self.start_time.elapsed().as_secs();
        format!("OK,{},{}", self.channel_id.channel_id, uptime)
    }

    fn handle_get_status(&self) -> String {
        format!("OK,{},0,0", self.channel_id.channel_id)
    }

    fn handle_reset_system(&mut self) -> String {
        // Reset device state to defaults
        self.frequency = 2400.0;
        self.rf_output_enabled = false;
        self.phase = 0.0;
        self.power_dbm = 10.0;
        self.power_watt = 0.01;
        self.attenuation = 20.0;
        self.magnitude = 0.5;
        self.start_time = Instant::now();
        "OK".to_string()
    }

    // New command handlers

    fn handle_get_power_dbm(&self) -> String {
        format!("OK,{},{:.2}", self.channel_id.channel_id, self.power_dbm)
    }

    fn handle_set_power_dbm(&mut self, parts: Vec<&str>) -> String {
        if parts.len() < 3 {
            return "ERROR: Invalid command format".to_string();
        }
        
        match parts[2].parse::<f64>() {
            Ok(power) => {
                self.power_dbm = power;
                // Update watts based on dBm
                self.power_watt = 10.0_f64.powf(self.power_dbm / 10.0) / 1000.0;
                "OK".to_string()
            },
            Err(_) => "ERROR: Invalid power value".to_string(),
        }
    }

    fn handle_get_power_watt(&self) -> String {
        format!("OK,{},{:.6}", self.channel_id.channel_id, self.power_watt)
    }

    fn handle_set_power_watt(&mut self, parts: Vec<&str>) -> String {
        if parts.len() < 3 {
            return "ERROR: Invalid command format".to_string();
        }
        
        match parts[2].parse::<f64>() {
            Ok(power) => {
                self.power_watt = power;
                // Update dBm based on watts
                self.power_dbm = 10.0 * (self.power_watt * 1000.0).log10();
                "OK".to_string()
            },
            Err(_) => "ERROR: Invalid power value".to_string(),
        }
    }

    fn handle_get_attenuation(&self) -> String {
        format!("OK,{},{:.2}", self.channel_id.channel_id, self.attenuation)
    }

    fn handle_set_attenuation(&mut self, parts: Vec<&str>) -> String {
        if parts.len() < 3 {
            return "ERROR: Invalid command format".to_string();
        }
        
        match parts[2].parse::<f64>() {
            Ok(att) => {
                self.attenuation = att;
                "OK".to_string()
            },
            Err(_) => "ERROR: Invalid attenuation value".to_string(),
        }
    }

    fn handle_get_magnitude(&self) -> String {
        format!("OK,{},{:.2}", self.channel_id.channel_id, self.magnitude)
    }

    fn handle_set_magnitude(&mut self, parts: Vec<&str>) -> String {
        if parts.len() < 3 {
            return "ERROR: Invalid command format".to_string();
        }
        
        match parts[2].parse::<f64>() {
            Ok(mag) => {
                self.magnitude = mag;
                "OK".to_string()
            },
            Err(_) => "ERROR: Invalid magnitude value".to_string(),
        }
    }

    fn handle_get_pa_temp(&self) -> String {
        format!("OK,{},{}", self.channel_id.channel_id, self.temperature)
    }

    fn handle_get_pa_voltage(&self) -> String {
        format!("OK,{},{}", self.channel_id.channel_id, self.voltage)
    }

    fn handle_get_pa_current(&self) -> String {
        format!("OK,{},{}", self.channel_id.channel_id, self.current)
    }

    fn handle_get_channel_id(&self) -> String {
        format!("OK,{}", self.channel_id.channel_id)
    }

    fn handle_set_channel_id(&mut self, parts: Vec<&str>) -> String {
        if parts.len() < 3 {
            return "ERROR: Invalid command format".to_string();
        }
        
        match parts[2].parse::<u8>() {
            Ok(id) => {
                self.channel_id = Channel { channel_id: id };
                "OK".to_string()
            },
            Err(_) => "ERROR: Invalid channel ID value".to_string(),
        }
    }
}





