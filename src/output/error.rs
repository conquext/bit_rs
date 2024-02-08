use std::fmt::Display;

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    InvalidName,    
    Undefined(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidName => "invalid name".to_string(),
                Error::Undefined(error) => format!("undefined error: {error}")
            }
        )
    }
}