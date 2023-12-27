use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use fatcrab_trading::error::FatCrabError as InnerError;

#[derive(Debug)]
pub enum FatCrabError {
    TxNotFound,
    TxUnconfirmed,
    Simple { description: String },
    N3xb { description: String },
    BdkBip39 { description: String },
    Bdk { description: String },
    Io { description: String },
    JoinError { description: String },
    SerdesJson { description: String },
    UrlParse { description: String },
}

impl Error for FatCrabError {}

impl From<InnerError> for FatCrabError {
    fn from(e: InnerError) -> Self {
        match e {
            InnerError::TxNotFound => Self::TxNotFound,
            InnerError::TxUnconfirmed => Self::TxUnconfirmed,
            InnerError::Simple { description } => Self::Simple { description },
            InnerError::N3xb { error } => Self::N3xb {
                description: error.to_string(),
            },
            InnerError::BdkBip39 { error } => Self::BdkBip39 {
                description: error.to_string(),
            },
            InnerError::Bdk { error } => Self::Bdk {
                description: error.to_string(),
            },
            InnerError::Io { error } => Self::Io {
                description: error.to_string(),
            },
            InnerError::JoinError { error } => Self::JoinError {
                description: error.to_string(),
            },
            InnerError::SerdesJson { error } => Self::SerdesJson {
                description: error.to_string(),
            },
        }
    }
}

impl Display for FatCrabError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let error_string: String = match self {
            FatCrabError::TxNotFound => "FatCrab-Error | TxNotFound".to_string(),
            FatCrabError::TxUnconfirmed => "FatCrab-Error | TxUnconfirmed".to_string(),
            FatCrabError::Simple { description } => {
                format!("FatCrab-Error | Simple - {}", description)
            }
            FatCrabError::N3xb { description } => format!("FatCrab-Error | n3xb - {}", description),
            FatCrabError::BdkBip39 { description } => {
                format!("FatCrab-Error | bip39 - {}", description)
            }
            FatCrabError::Bdk { description } => format!("FatCrab-Error | bdk - {}", description),
            FatCrabError::Io { description } => format!("FatCrab-Error | Io - {}", description),
            FatCrabError::JoinError { description } => {
                format!("FatCrab-Error | JoinError - {}", description)
            }
            FatCrabError::SerdesJson { description } => {
                format!("FatCrab-Error | SerdesJson - {}", description)
            }
            FatCrabError::UrlParse { description } => {
                format!("FatCrab-Error | UrlParse - {}", description)
            }
        };
        write!(f, "{}", error_string)
    }
}

impl From<tokio::task::JoinError> for FatCrabError {
    fn from(e: tokio::task::JoinError) -> FatCrabError {
        FatCrabError::JoinError {
            description: e.to_string(),
        }
    }
}

impl From<url::ParseError> for FatCrabError {
    fn from(e: url::ParseError) -> FatCrabError {
        FatCrabError::UrlParse {
            description: e.to_string(),
        }
    }
}
