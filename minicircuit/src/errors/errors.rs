use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MWError {
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
    FailedParseResponse,
}

// impl from string for mWerror

impl From<String> for MWError {
    fn from(value: String) -> Self {
        todo!();
        match value.as_str() {
            "ERR01" => MWError::Reserved,
            _ => MWError::FailedParseResponse,
        }
    }
}

impl MWError {
    pub fn new(error_code: String) -> Self {
        let e = error_code;

        if e.contains("ERR01") {
            MWError::Reserved
        } else if e.contains("ERR02") {
            MWError::MaxLengthExceeded
        } else if e.contains("ERR03") {
            MWError::TooFewArgs
        } else if e.contains("ERR04") {
            MWError::TooManyArgs
        } else if e.contains("ERR05") {
            MWError::WrongMode
        } else if e.contains("ERR06") {
            MWError::SystemBusy
        } else if e.contains("ERR07") {
            MWError::SatisfiedNotImpl
        } else if e.contains("ERR10") {
            MWError::ArgNumber
        } else if e.contains("ERR11") {
            MWError::InvalidArg { arg: 1 }
        } else if e.contains("ERR12") {
            MWError::InvalidArg { arg: 2 }
        } else if e.contains("ERR13") {
            MWError::InvalidArg { arg: 3 }
        } else if e.contains("ERR14") {
            MWError::InvalidArg { arg: 4 }
        } else if e.contains("ERR15") {
            MWError::InvalidArg { arg: 5 }
        } else if e.contains("ERR16") {
            MWError::InvalidArg { arg: 6 }
        } else if e.contains("ERR17") {
            MWError::InvalidArg { arg: 7 }
        } else if e.contains("ERR18") {
            MWError::InvalidArg { arg: 8 }
        } else if e.contains("ERR19") {
            MWError::InvalidArg { arg: 9 }
        } else if e.contains("ERR7E") {
            MWError::FailedExe
        } else {
            MWError::FailedParseResponse
        }
    }
}

impl Error for MWError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for MWError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MWError::Reserved => todo!(),
            MWError::MaxLengthExceeded => todo!(),
            MWError::TooFewArgs => todo!(),
            MWError::TooManyArgs => todo!(),
            MWError::WrongMode => todo!(),
            MWError::SystemBusy => todo!(),
            MWError::SatisfiedNotImpl => todo!(),
            MWError::ArgNumber => todo!(),
            MWError::InvalidArg { arg } => todo!(),
            MWError::FailedExe => todo!(),
            MWError::FailedParseResponse => todo!(),
            // FrcError::Serialization(ref msg) => write!(f, "Serialization error: {}", msg),
            // FrcError::UnrecognizedPacket => write!(f, "Fanuc threw an unrecognized weeoe"),
            // FrcError::FanucErrorCode(ref code) => write!(f, "fanuc returned  error#: {}", code.message()),
            // FrcError::FailedToSend(ref msg) => write!(f, "SendError: {}", msg),
            // FrcError::FailedToRecieve(ref msg) => write!(f, "RecieveError: {}", msg),
            // FrcError::Disconnected() => write!(f, "Fanuc appears to be disconnected"),
            // FrcError::Initialization(ref msg) => write!(f, "Could not initialize: {}", msg)
        }
    }
}
