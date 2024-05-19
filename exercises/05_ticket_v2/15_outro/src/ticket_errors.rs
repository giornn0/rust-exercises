use thiserror::Error;

use crate::status::ParseStatusError;

#[derive(Debug, Error)]
pub enum TicketError {
    #[error("The {0} cannot be empty")]
    Empty(String),
    #[error("The {0} cannot be longer than {1} characters")]
    Longer(String, i32),
    #[error("{invalid_status}")]
    ParseStatus {
        #[from]
        #[source]
        invalid_status: ParseStatusError,
    },
}
