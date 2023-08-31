use crate::types::ErrorInfo;
use thiserror::Error as ThisError;


#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum Error {

    #[error("NotFound")]
    NotFound,

    #[error("Http request error")]
    RequestError,
    
}