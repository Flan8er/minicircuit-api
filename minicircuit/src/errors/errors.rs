pub enum Error {
    Reserved,
    MaxLengthExceeded,
    TooFewArgs,
    TooManyArgs,
    WrongMode,
    SystemBusy,
    SatisfiedNotImpl,
    ArgNumber,
    InvalidArg { arg: u16 },
    FailedExe,
    Unknown,
}

impl Error {
    pub fn new(error_code: String) -> Self {
        let e = error_code;

        if e.contains("ERR01") {
            Error::Reserved
        } else if e.contains("ERR02") {
            Error::MaxLengthExceeded
        } else if e.contains("ERR03") {
            Error::TooFewArgs
        } else if e.contains("ERR04") {
            Error::TooManyArgs
        } else if e.contains("ERR05") {
            Error::WrongMode
        } else if e.contains("ERR06") {
            Error::SystemBusy
        } else if e.contains("ERR07") {
            Error::SatisfiedNotImpl
        } else if e.contains("ERR10") {
            Error::ArgNumber
        } else if e.contains("ERR11") {
            Error::InvalidArg { arg: 1 }
        } else if e.contains("ERR12") {
            Error::InvalidArg { arg: 2 }
        } else if e.contains("ERR13") {
            Error::InvalidArg { arg: 3 }
        } else if e.contains("ERR14") {
            Error::InvalidArg { arg: 4 }
        } else if e.contains("ERR15") {
            Error::InvalidArg { arg: 5 }
        } else if e.contains("ERR16") {
            Error::InvalidArg { arg: 6 }
        } else if e.contains("ERR17") {
            Error::InvalidArg { arg: 7 }
        } else if e.contains("ERR18") {
            Error::InvalidArg { arg: 8 }
        } else if e.contains("ERR19") {
            Error::InvalidArg { arg: 9 }
        } else if e.contains("ERR7E") {
            Error::FailedExe
        } else {
            Error::Unknown
        }
    }
}

pub enum Status {
    Nominal,
    Unspecified,
    PATempHigh,
    PAShutdownTemp,
    HighReflection,
    ShutdownReflection,
    ResetDetected,
    ReadoutError,
    PowerMeasurementFailure,
    RFOutputFailure,
    MultiplexerFailure,
    ExternalShutdown,
    Reserved,
    I2CComsFailure,
    SPIComsFailure,
    IQConversionError,
    SOAMeasurementError,
    WatchdogTimeout,
    MissingCalibration,
    SOAHighDissipation,
    SOAShutdownDissipation,
    IncompatibleFirmware,
    InternalPAError,
    PAResetFailure,
    HighCurrent,
}

impl Status {
    pub fn new(hex_value: u32) -> Self {
        match hex_value {
            0 => Self::Nominal,
            1 => Self::Unspecified,
            2 => Self::PATempHigh,
            4 => Self::PAShutdownTemp,
            8 => Self::HighReflection,
            10 => Self::ShutdownReflection,
            20 => Self::ResetDetected,
            40 => Self::ReadoutError,
            80 => Self::PowerMeasurementFailure,
            100 => Self::RFOutputFailure,
            200 => Self::MultiplexerFailure,
            400 => Self::ExternalShutdown,
            800 => Self::Reserved,
            1000 => Self::I2CComsFailure,
            2000 => Self::SPIComsFailure,
            4000 => Self::IQConversionError,
            8000 => Self::SOAMeasurementError,
            10000 => Self::WatchdogTimeout,
            20000 => Self::MissingCalibration,
            40000 => Self::Reserved,
            80000 => Self::SOAHighDissipation,
            100000 => Self::SOAShutdownDissipation,
            200000 => Self::IncompatibleFirmware,
            400000 => Self::InternalPAError,
            800000 => Self::PAResetFailure,
            1000000 => Self::HighCurrent,
            _ => Self::Unspecified,
        }
    }
}
