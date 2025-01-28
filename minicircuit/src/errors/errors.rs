use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MWError {
    /// Error code is reserved.
    Reserved,
    /// The serial message exceeded the maximum length.
    MaxLengthExceeded,
    /// The serial message had too few arguments.
    TooFewArgs,
    /// The serial message had too many arguments.
    TooManyArgs,
    /// The system could not accept this message is the current mode.
    WrongMode,
    /// The system was busy and cannot process this message at this time.
    SystemBusy,
    /// The message was recognized but is not yet implemented in the codebase.
    SatisfiedNotImpl,
    /// An argument was in error with the lower nibble indicating the argument number.
    ArgNumber,
    /// Argument was invalid / out of range.
    InvalidArg { arg: u16 },
    /// Command execution failed.
    FailedExe,
    /// An error occurred that is not covered by any of the other error codes.
    Unknown,
    /// An error occurred parsing the response to the given command.
    FailedParseResponse,
}

impl From<String> for MWError {
    fn from(value: String) -> Self {
        let trimmed_error = trim_before_err(&value).trim();

        match trimmed_error {
            "ERR01" => Self::Reserved,
            "ERR02" => Self::MaxLengthExceeded,
            "ERR03" => Self::TooFewArgs,
            "ERR04" => Self::TooManyArgs,
            "ERR05" => Self::WrongMode,
            "ERR06" => Self::SystemBusy,
            "ERR07" => Self::SatisfiedNotImpl,
            "ERR10" => Self::ArgNumber,
            "ERR11" => Self::InvalidArg { arg: 1 },
            "ERR12" => Self::InvalidArg { arg: 2 },
            "ERR13" => Self::InvalidArg { arg: 3 },
            "ERR14" => Self::InvalidArg { arg: 4 },
            "ERR15" => Self::InvalidArg { arg: 5 },
            "ERR16" => Self::InvalidArg { arg: 6 },
            "ERR17" => Self::InvalidArg { arg: 7 },
            "ERR18" => Self::InvalidArg { arg: 8 },
            "ERR19" => Self::InvalidArg { arg: 9 },
            "ERR7E" => Self::FailedExe,
            "ERR7F" => Self::Unknown,
            _ => MWError::FailedParseResponse,
        }
    }
}

fn trim_before_err(input: &str) -> &str {
    if let Some(pos) = input.find("ERR") {
        &input[pos..]
    } else {
        input // If "ERR" is not found, return the original string
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
            Self::Reserved => write!(f, "Reserved error."),
            Self::MaxLengthExceeded => write!(f, "The serial message exceeded the maximum length."),
            Self::TooFewArgs => write!(f, "The serial message had too few arguments."),
            Self::TooManyArgs => write!(f, "The serial message had too many arguments."),
            Self::WrongMode => write!(
                f,
                "The system could not accept this message is the current mode."
            ),
            Self::SystemBusy => write!(
                f,
                "The system was busy and cannot process this message at this time."
            ),
            Self::SatisfiedNotImpl => write!(
                f,
                "The message was recognized but is not yet implemented in the codebase."
            ),
            Self::ArgNumber => write!(
                f,
                "An argument was in error with the lower nibble indicating the argument number."
            ),
            Self::InvalidArg { arg } => write!(f, "Argument {} was invalid / out of range.", arg),
            Self::FailedExe => write!(f, "Command execution failed."),
            Self::Unknown => write!(
                f,
                "An error occurred that is not covered by any of the other error codes."
            ),
            Self::FailedParseResponse => write!(
                f,
                "An error occurred parsing the response to the given command."
            ),
        }
    }
}
