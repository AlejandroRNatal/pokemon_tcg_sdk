use std::{ fmt::Formatter };

use derive_more::{ From };
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {
    MissingArgument{ arg: String },
    MissingSetMapping { set: String },

    
    FailedOpeningFile,
    FailedParsingFile,

    ApiKeyNotFound,

    InvalidArgument {
        arg: String,
    },
    InvalidEndpoint {
        url: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Error {
        Self::InvalidArgument{ arg: val.to_owned() }
    }
}

impl std::error::Error for Error { }
