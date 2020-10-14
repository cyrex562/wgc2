use std::fmt;

///
///
#[derive(Debug, Clone, Default)]
pub struct MultiError {
    pub kind: String,
    pub message: String,
}

impl fmt::Display for MultiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid command line")
    }
}

impl From<std::num::ParseIntError> for MultiError {
    fn from(error: std::num::ParseIntError) -> Self {
        MultiError {
            kind: String::from("ParseIntError"),
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for MultiError {
    fn from(error: std::io::Error) -> Self {
        MultiError {
            kind: String::from("IoError"),
            message: error.to_string(),
        }
    }
}

impl From<csv::Error> for MultiError {
    fn from(error: csv::Error) -> Self {
        MultiError {
            kind: String::from("CsvError"),
            message: error.to_string(),
        }
    }
}

impl From<fern::InitError> for MultiError {
    fn from(error: fern::InitError) -> Self {
        MultiError {
            kind: String::from("FernInitError"),
            message: error.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for MultiError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        MultiError {
            kind: String::from("FromUtf8Error"),
            message: error.to_string(),
        }
    }
}
