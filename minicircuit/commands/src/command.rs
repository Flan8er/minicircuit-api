use serde::{Deserialize, Serialize};

use crate::{
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
    /// Returns the forward and reflected power as ADC counts.
    ///
    /// Depending on the PA Type, these ADC counts are either converted from the analog voltage inputs on the ISC board,
    /// or from the ADCs on the ZHL-2425-250X+ (See `SetPAType`). If the source of the ADC count is the ISC board,
    /// ADC measurements are averaged over 10 samples. Otherwise, a single sample is returned.
    GetPAPowerADC(GetPAPowerADC),
    /// Returns the DC current reading of the ISC in Amps.
    GetPACurrent(GetPACurrent),
    /// Returns the forward and reflected power of the power amplifier in dBm.
    GetPAPowerDBM(GetPAPowerDBM),
    /// Returns the forward and reflected power in watts.
    GetPAPowerWatt(GetPAPowerWatt),
    /// Returns the frequency of the ISC board's RF output in MHz.
    GetFrequency(GetFrequency),
    /// Sets the frequecy of the ISC board's RF output to the desired value in MHz.
    SetFrequency(SetFrequency),
    /// Returns the enable state of the ISC board's RF output.
    ///
    /// Enable state can be set with `SetRFOutput`, but there are also many status
    /// conditions that turn RF output OFF for safety reasons. Check `GetStatus` for details.
    GetRFOutput(GetRFOutput),
    /// Turns RF output of the ISC board ON or OFF.
    ///
    /// Board is turned off by default.
    SetRFOutput(SetRFOutput),
    /// Returns the current phase value of the ISC board's RF output in degrees.
    GetPhase(GetPhase),
    /// Sets the phase of the ISC board's RF output in degrees.
    ///
    /// The phase set is reference to the selected clock source (see ClockSource).
    SetPhase(SetPhase),
    /// Returns the configured output power setpoint in dBm.
    GetPAPowerSetpointDBM(GetPAPowerSetpointDBM),
    /// Returns the configured output power setpoint in watts.
    GetPAPowerSetpointWatt(GetPAPowerSetpointWatt),
    /// Sets the output power setpoint to the desired value in dBm.
    SetPAPowerSetpointDBM(SetPAPowerSetpointDBM),
    /// Sets the amplifier chain's output power setpoint to the desired value in watts.
    SetPAPowerSetpointWatt(SetPAPowerSetpointWatt),
    /// Returns the temperature of the power amplifier (PA).
    GetPATemp(GetPATemp),
    /// Returns the measured DC voltage of the PA in Volts.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPAVoltage(GetPAVoltage),
    /// Returns the configured parameters of the DLL mode.
    GetDLLConfig(GetDLLConfig),
    /// Sets the configured parameters of the DLL mode.
    SetDLLConfig(SetDLLConfig),
    /// Returns the state of DLL mode - either turned ON or OFF
    GetDLLEnabled(GetDLLEnabled),
    /// Turns DLL mode ON or OFF
    ///
    /// True = On,
    /// False = Off (default)
    SetDLLEnabled(SetDLLEnabled),
    /// Output's the best frequency to be at given the requested power output.
    ///
    /// Performs an S11 frequency sweep across the band provided.
    ///
    /// The completion time of the command will increase as the number of frequency steps increases.
    /// This can make it seem as if the ISC board has become un-responsive for some time.
    PerformSweepDBM(PerformSweepDBM),
    /// Output's the best frequency to be at given the requested power output.
    ///
    /// Performs an S11 frequency sweep across the band provided.
    ///
    /// The completion time of the command will increase as the number of frequency steps increases.
    /// This can make it seem as if the ISC board has become un-responsive for some time.
    PerformSweepWatt(PerformSweepWatt),
    /// Clears the error state of the ISC board and resets the protective systems
    /// that impede the board while an error is present.
    ClearErrors(ClearErrors),
    /// Gets the status of the power amplifier (PA). If the status is 0, this indicates normal operation.
    /// If the status is non-zero, one or more PA internal protection limits have been triggered.
    /// Typically, this means that the PA will have already shut itself down in self-protection.
    /// When the PA error code of a system in non-zero, it raises the `PAError` and triggers SOA `PAStatus`.
    /// If an alarm signal is sent from the PA to the ISC, the `AlarmIn` error will also be raised. In multi-channel systems,
    /// the returned error code status is a bitwise OR of the statuses of each channel.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPAErrors(GetPAErrors),
    /// Used to monitor the status of the ISC board.
    ///
    /// ISC boards have a safety feature called the 'Safe Operating Area' (SOA).
    /// If a fault occurs during operation, the SOA raises an error and takes action to protect the system.
    /// This is indicated by a red LED on the board.
    /// An error on the ISC board is accompanied by an informative error code which can be used to trace the problem.
    ///
    /// To ensure the code can be viewed it stays in memory until manually cleared away.
    /// For safety reasons some errors block the RF power output of the ISC board until cleared (ClearErrors).
    GetStatus(GetStatus),
    /// Returns the identity of the ISC board.
    GetIdentity(GetIdentity),
    /// Returns the temperature of the microcontroller on the ISC board.
    GetISCTemp(GetISCTemp),
    /// Returns the uptime of the ISC board since its initialization.
    /// The uptime count restarts when the board is reset.
    GetUptime(GetUptime),
    /// Returns the current version of the firmware.
    GetVersion(GetVersion),
    /// Returns the configured attenuation value of the VGA which regulates the ISC board's power output.
    /// The higher the value, the lower the power output.
    GetAttenuation(GetAttenuation),
    /// TO USE THIS COMMAND, `SetAutoGain` MUST BE DISABLED FIRST
    ///
    /// Set the attenuation of the variable gain amplifier (VGA) which regulates
    /// the ISC board's power output.
    /// The higher the value, the lower the power output.
    ///
    /// Under normal conditions, both the VGA and the IQ modulator are used to regulate power output of the ISC board,
    /// thus the actual power output is a combination of both.
    /// The IQ modulator is controlled using the SetQIMagPercent command.
    SetAttenuation(SetAttenuation),
    /// Returns the enable state of the auto-gain algorithm.
    GetAutoGainState(GetAutoGainState),
    /// Turns the auto-gain algorithm ON or OFF.
    ///
    /// The auto-gain algorithm automatically regulates the power output of the ISC board by configuring the DSA and Modulator bias
    /// according to calibrations that are stored in the device's EEPROM and feedback from the PA.
    ///
    /// When auto-gain is enabled, the user can simply request an arbitrary amount of power (in Watt / dBm)
    /// from their RF system, and the requested power will be accurately generated (as long
    /// as the calibration is good and there are no unexpected interferences).
    ///
    /// When auto-gain is disabled, the user can take manual control of the DSA and Modulator bias.
    /// Operating manually is not recommended in most situations but can be useful for troubleshooting
    /// and characterizing RF systems.
    ///
    /// Disabling auto-gain has consequences for a variety of commands:
    ///
    /// - `SetPAPowerSetpointDBM` and `SetPAPowerSetpointWatt`set the DSA state according to the static feed-forward calibration
    /// stored in the EEPROM.
    ///
    /// - Power can be regulated manually using commands like `SetQIMagPercent` and `SetVGAAttenuationDB` to control
    /// the DSA and Modulator bias directly.
    ///
    /// - `PerformSweepWatt` and `PerformSweepDBM` ignore the "Sweet Power" argument. Sweeps are performed at whatever power output is configured
    /// through the DSA and IQ modulator at the time.
    SetAutoGainState(SetAutoGainState),
    /// Gets the magnitude of the IQ modulator.
    GetMagnitude(GetMagnitude),
    /// TO USE THIS COMMAND, `SetAutoGain` MUST BE DISABLED FIRST
    ///
    /// This command sets the magnitude setting of the IQ modulator, which regulates the ISC board's power output.
    /// The higher the value, the higher the power output.
    ///
    /// Remark: Under normal conditions, both the VGA and the IQ modulator are used to regulate the power output of the ISC board,
    /// thus the actual power output is a combination of both.
    SetMagnitude(SetMagnitude),
    /// Returns the last power set. The last power set does not indicate
    /// the current state of the VGA and IQ Modulator which could have changed due to
    /// calls to `SetMagnitude`, `SetAttenuation`, or any other function
    /// that affects these settings.
    GetISCPowerOutput(GetISCPowerOutput),
    /// TO USE THIS COMMAND, `SetAutoGain` MUST BE DISABLED FIRST
    ///
    /// Provides a coarse method to regulate the small signal output power of the
    /// ISC board by automatically configuring the values of the VGA and IQ modulator
    /// to the roughly desired dBm value.
    SetISCPowerOutput(SetISCPowerOutput),
    /// Returns all the settings relating to PWM.
    GetPWMDutyCycle(GetPWMDutyCycle),
    /// Sets the PWM duty cycle between 0% and 100%.
    ///
    /// This command doubles as a PWM ON/OFF switch. Setting the duty cycle
    /// to 100% is the same as turning PWN off entirely, thus there is no
    /// dedicated PWM ON/OFF command.
    SetPWMDutyCycle(SetPWMDutyCycle),
    /// Sets the frequency of the PWM signal.
    SetPWMFrequency(SetPWMFrequency),
    /// Initiates a single timed enable of specified duration.
    SetTimedRFEnable(SetTimedRFEnable),
    /// Returns the enable state of the SOA's protection systems.
    GetSOAConfig(GetSOAConfig),
    /// Configures the enable state of the SOA's protection systems.
    ///
    /// SOA has the following protection systems in place:
    ///
    /// - Protection against high temperatures.
    ///
    /// - Protections against software timeouts / freezes.
    ///
    /// - Protection against excessive reflection.
    ///
    /// - Auto-disable RF power if the board status is not polled frequently enough.
    SetSOAConfig(SetSOAConfig),
    /// Returns the currents at which SOA takes action.
    ///
    /// One of the features of the SOA is protection against improper
    /// application of DC current. Current SOA protects against overcurrent conditions.
    ///
    /// The SOA has two reactions to excessive current, depending on the severity:
    ///
    /// - If the current is higher than normal operating range, but still tolerable: raise a `SOAHighCurrent` error.
    ///
    /// - If the current is dangerously high: raise a `SOAShutdownMaximumCurrent` error and shutdown RF power.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetSOACurrentConfig(GetSOACurrentConfig),
    /// Sets the currents at which SOA takes action.
    ///
    /// One of the features of the SOA is protection against improper
    /// application of DC current. Current SOA protects against overcurrent conditions.
    ///
    /// The SOA has two reactions to excessive current, depending on the severity:
    ///
    /// - If the current is higher than normal operating range, but still tolerable: raise a `SOAHighCurrent` error.
    ///
    /// - If the current is dangerously high: raise a `SOAShutdownMaximumCurrent` error and shutdown RF power.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOACurrentConfig(SetSOACurrentConfig),
    /// Returns the dissipation at which SOA takes action in Watts.
    ///
    /// One of the features of the SOA is protection against excessive power dissipation inside a generator.
    /// Excessive power dissipation occurs when an RF system draws a disproportionate amount of current from it's
    /// power supply (PSU) relative to the amount RF energy that is transmitted into a load. High dissipation
    /// can be reached when the system is poorly matched or when the system is well matched but still operating
    /// with poor efficiency. At the system level, dissipation is the rate that heat needs to be removed from the
    /// generator by means of heat sink or cooling plate to maintain a stable temperature. The dissipation SOA
    /// could be used in systems with limited cooling capacity to issue a warning to the user to shut the generator
    /// down before it has a change to heat up to the temperature shutdown limit.
    GetSOADissipationConfig(GetSOADissipationConfig),
    /// Sets the dissipation at which SOA takes action in Watts.
    ///
    /// One of the features of the SOA is protection against excessive power dissipation inside a generator.
    /// Excessive power dissipation occurs when an RF system draws a disproportionate amount of current from it's
    /// power supply (PSU) relative to the amount RF energy that is transmitted into a load. High dissipation
    /// can be reached when the system is poorly matched or when the system is well matched but still operating
    /// with poor efficiency. At the system level, dissipation is the rate that heat needs to be removed from the
    /// generator by means of heat sink or cooling plate to maintain a stable temperature. The dissipation SOA
    /// could be used in systems with limited cooling capacity to issue a warning to the user to shut the generator
    /// down before it has a change to heat up to the temperature shutdown limit.
    SetSOADissipationConfig(SetSOADissipationConfig),
    /// Returns the forward power values at which SOA takes action in Watts.
    ///
    /// One of the features of the SOA is protection against excessive forward power.
    ///
    /// The SOA has two reactions to excess forward power, depending on the severity:
    ///
    /// - If the forward power is high, but still tolerable: raise a `HighForwardPower` error.
    ///
    /// - If the forward power is dangerously high: raise a `ShutdownForwardPower` error and shutdown RF power.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetSOAForwardPowerLimits(GetSOAForwardPowerLimits),
    /// Sets the forward power values at which SOA takes action in Watts.
    ///
    /// One of the features of the SOA is protection against excessive forward power.
    ///
    /// The SOA has two reactions to excess forward power, depending on the severity:
    ///
    /// - If the forward power is high, but still tolerable: raise a `HighForwardPower` error.
    ///
    /// - If the forward power is dangerously high: raise a `ShutdownForwardPower` error and shutdown RF power.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOAForwardPowerLimits(SetSOAForwardPowerLimits),
    /// Configures the grace period for the SOA's protection systems.
    ///
    /// There may be situations where it is desirable to permit a grace period before SOA acts
    /// and potentially shuts down everything. The SOA grace timer may be used to allow temporary violations
    /// of the reflection, dissipation, and temperature limits for a configurable period. Only a continuous,
    /// uninterrupted violation longer than the grace timeout will trigger a reaction from the SOA.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOAGraceTimer(SetSOAGraceTimer),
    /// Returns the reflection values at which SOA takes action.
    GetSOAPowerConfig(GetSOAPowerConfig),
    /// Configures the reflected power values at which SOA takes action.
    /// One of the features of SOA is protection against excessive reflected power.
    /// Excessive reflection occurs when there is a bad match at the output and RF returns to the generator.
    ///
    /// The SOA has two reactions to excessive dissipation, depending on the severity:
    ///
    /// - If the reflection is high, but still tolerable: raise a 'HighReflection' error.
    ///
    /// - If the reflection is dangerously high: raise a 'ShutdownReflection' error and shutdown RF power.
    SetSOAPowerConfig(SetSOAPowerConfig),
    /// Returns the temperature values at which the SOA takes action.
    GetSOATempConfig(GetSOATempConfig),
    /// Configures the temperature values at which SOA takes action.
    /// One of the features of the SOA is protection against excessive temperatures.
    /// Excessive temperatures can occur for any number of reasons: side effects of high
    /// RF power reflection, faulty cooling, excessive use, etc.
    ///
    /// The SOA has two reactions to excessive temperatures, depending on the severity:
    ///
    /// - If the temperature is high, but still tolerable: raise a `HighTemperature` error.
    ///
    /// - If the temperature is dangerously high: raise a `ShutdownTemperature` error and shutdown RF power.
    SetSOATempConfig(SetSOATempConfig),
    /// Returns the enable state of the SOA's protection systems.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetSOAVoltageConfig(GetSOAVoltageConfig),
    /// Sets the voltages at which the SOA takes action. One of the features of the SOA
    /// is protection against improper application of DC voltage. Voltage SOA protects
    /// against both undervoltage and overvoltage conditions.
    ///
    /// The SOA has two reactions to excessive voltage, depending on the severity:
    ///
    /// - If the voltage is outside of the normal operating range, but still tolerable: raise a `SOAHighVoltage` or `SOALowVoltage` error.
    ///
    /// - If the voltage is dangerously low or high: raise a `SOAShutdownMinimumVoltage` or `SOAShutdownMaximumVoltage` error and shutdown RF power.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOAVoltageConfig(SetSOAVoltageConfig),
    /// Turns the software watchdog ON or OFF
    ///
    /// The software watchdog is a function of the firmware which ensures that the various software components of the
    /// firmware keep working as intended.
    ///
    /// The following software components are guarded by the watchdog:
    ///
    /// - Serial command interpreter / UART bus
    ///
    /// - I^2C bus
    ///
    /// - PWM trigger
    ///
    /// - Safe Operating Area
    ///
    /// - Auto-gain
    ///
    /// - DLL algorithm
    ///
    /// - USB bus
    ///
    /// - Debug thread
    ///
    /// The software watchdog sends requests to each of the components to confirm whether they
    /// are still running. If the component fails to respond too many times in a row,
    /// the watchdog triggers and the ISC board is automatically reset.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetSOAWatchdogConfig(SetSOAWatchdogConfig),
    /// THIS COMMAND DOES NOT REPLY.
    ///
    /// Sets the baud rate used for communicating through UART.
    /// Any value can be entered, but unsurprisingly, ongoing
    /// communication will break the moment this value is changed.
    ///
    /// Changing the baud rate affects communication speed. Lowering it
    /// can cause noticable communication delays, while increasing it can
    /// speed up communication and leave a larger CPU time-slice for
    /// other tasks. However, setting the baud rate too high may cause
    /// communication issues to arise, as the UART transceivers have limitations.
    ///
    /// After changing the baud rate, the communication line needs to be reinitialized
    /// on the user side with the updated baud values.
    ///
    /// This setting does not affect communication through USB, only through UART.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetUartBaudRate(SetUartBaudRate),
    /// Returns the channel number assigned to the ISC board.
    GetChannelID(GetChannelID),
    /// Assigns a channel identification number to the specified ISC board.
    ///
    /// Every ISC board is assigned a numeric value as a challen identifier for communication.
    /// The default value of the identifier is `1`, which serves its purpose in single-channel systems.
    /// In setups that deploy more than one ISC board is often necessary to assign a unique number to each individual board beforehand,
    /// so that they can all be commanded as seperate entities. An ISC board will not respond to commands written for a different channel.
    SetChannelID(SetChannelID),
    /// Returns the clock source configuration of the ISC board.
    GetClockSource(GetClockSource),
    /// Sets the clock source configuration (or "coherency mode") of the ISC board.
    ///
    /// An ISC board can either use its own internal 10MHz Crystal Controlled Oscillator (XCO),
    /// or it can accept an external clock reference from another ISC board.
    /// The clock signal can be transmitted and received using a Low Voltage Differential Signaling (LVDS) transceiver.
    ///
    /// The clock source is required to synchronize signal phase of ISC boards in
    /// coherent multi-channel systems.
    SetClockSource(SetClockSource),
    /// Sets the communication interface to UART (3.3V TTL) or USB. Only one communication
    /// interface can be active at a time.
    ///
    /// The default communication interface is USB. If the user switches to UART by sending a
    /// `SetCommunicationInterface::new(Channel::default(), Interface::Uart)` command, the USB serial
    /// port will no longer be active. COmmunication may only resume over UART during that session.
    ///
    /// Rebooting will return the unit back to its default communication interface (USB).
    SetCommunicationInterface(SetCommunicationInterface),
    /// Returns the maximum permitted forward power setting in dBm.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPowerMaxDbm(GetPowerMaxDbm),
    /// Configures a maximum output power cap. This prevents inputting a forward power setpoint
    /// (`SetPAPowerSetpointWatt` / `SetPAPowerSetpointDBM`) beyond the configured maximum value.
    /// Useful for configuring or ignoring limits in special situations.
    SetPowerMaxDbm(SetPowerMaxDbm),
    /// Returns the minimum permitted forward power setting in dBm.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPowerMinDbm(GetPowerMinDbm),
    /// Configures a minimum output power cap. This limits the forward power setpoint
    /// (`SetPAPowerSetpointWatt` / `SetPAPowerSetpointDBM`) to be no lower than the configured minimum value.
    /// This minimum power limit ensures that power setting inputs stay within the valid calibration range of the instruments.
    /// This is especially important when operating in feed-forward mode where the internal
    /// attenuation settings are only well-defined for powers within the operating range.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetPowerMinDbm(SetPowerMinDbm),
    /// Returns the power offset of the system in dB.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    GetPowerOffset(GetPowerOffset),
    /// Sets the power offset of the system.
    ///
    /// Power offset is used when there is a fixed attenuation at the output
    /// of the generator and the user would like to see power referenced to the
    /// plane after that attenuation. For example, an offset setting of 3 would mean
    /// that there is 3bB of loss between the generator output and the new reference plane.
    ///
    /// This affects the behavior of several functions:
    ///
    /// - `GetPAPowerWatt` and `GetPAPowerDBM` normally return the forward and reflected powers.
    ///  Now forward powers are reduces by the offset value (in dB) and the reflected powers are
    /// increased by the offset value (in dB). Note that this means that any calculaton of Return
    /// loss will be 2 * offset (dB) lower than normal.
    ///
    /// - In both auto-gain and feed-forward modes, `SetPAPowerSetpointWatt` and `SetPAPowerSetpointDBM`
    /// are now referencing the power at the new reference plane. The minimum and maximum power settings
    /// are adjusted accordingly (reduced by the offset).
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetPowerOffset(SetPowerOffset),
    /// Executes a reset of the ISC board.
    /// All board settings will return to their default states.
    ///
    /// Following a reset, whether intentional or as the result of a fault,
    /// the `ResetDetected` error flag (0x20) will be raised.
    ResetSystem(ResetSystem),
    /// Sets the trigger delay on the ZHL in units of μs. Refer to the device data sheet
    /// for details on this parameter. The ISC board sends triggers to trigger measurements
    /// while PWM, DLL, or Sweep features are active. This delay parameter should generally not
    /// be changed.
    #[deprecated(
        note = "This function isn't implemented for the ISC-2425-25+ controller. If you're not using this controller, you can ignore this warning."
    )]
    SetZHLTriggerDelay(SetZHLTriggerDelay),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub priority: Priority,
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Standard,
    High,
    Immediate,
    Termination,
}
