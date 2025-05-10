use log::{debug, info};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use minicircuit_commands::data_types::types::Channel;

/// Simulates a MiniCircuit device by processing commands and generating responses
pub struct MiniCircuitSimulator {
    // Store device state
    frequency: f64,
    rf_output_enabled: bool,
    phase: f64,
    channel_id: Channel,
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
            command_log: Vec::new(),
        }
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
        
        match parts[0] {
            "$FCG" => self.handle_get_frequency(),
            "$FCS" => self.handle_set_frequency(parts),
            "$ECS" => self.handle_set_rf_output(parts),
            "$ECG" => self.handle_get_rf_output(),
            "$PCG" => self.handle_get_phase(),
            "$PCS" => self.handle_set_phase(parts),
            "$IDN" => self.handle_get_identity(),
            "$TCG" => self.handle_get_isc_temp(),
            "$RTG" => self.handle_get_uptime(),
            "$ST" => self.handle_get_status(),
            "$RST" => self.handle_reset_system(),
            // Add more command handlers as needed
            _ => format!("ERROR: Unknown command {}", parts[0]),
        }
    }

    fn handle_get_frequency(&self) -> String {
        format!("OK,{:.2}", self.frequency)
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
        format!("OK,{}", if self.rf_output_enabled { 1 } else { 0 })
    }

    fn handle_get_phase(&self) -> String {
        format!("OK,{:.2}", self.phase)
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
        "OK,MiniCircuits,ISC-2400-XX,SN12345678".to_string()
    }

    fn handle_get_isc_temp(&self) -> String {
        "OK,35.5".to_string() // Simulated temperature in Celsius
    }

    fn handle_get_uptime(&self) -> String {
        "OK,3600".to_string() // Simulated uptime in seconds
    }

    fn handle_get_status(&self) -> String {
        "OK,0,No errors".to_string()
    }

    fn handle_reset_system(&mut self) -> String {
        // Reset device state to defaults
        self.frequency = 2400.0;
        self.rf_output_enabled = false;
        self.phase = 0.0;
        "OK".to_string()
    }

    /// Get the command log
    pub fn get_command_log(&self) -> &Vec<String> {
        &self.command_log
    }
}