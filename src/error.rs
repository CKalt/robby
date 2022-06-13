use std::fmt;
use std::io;

#[derive(Debug)]
pub struct AppError {
    pub kind: String,     // type of the error
    pub message: String, // error message
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.message)
    }
}

// Implement std::convert::From for AppError; from io::Error
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

/*
// Implement std::convert::From for AppError; from csv::Error
impl From<csv::Error> for AppError {
    fn from(error: csv::Error) -> Self {
        AppError {
            kind: String::from("csv"),
            message: error.to_string(),
        }
    }
}

// Implement std::convert::From for AppError; from postgres::Error
impl From<postgres::Error> for AppError {
    fn from(error: postgres::Error) -> Self {
        AppError {
            kind: String::from("postgres"),
            message: error.to_string(),
        }
    }
}
*/
