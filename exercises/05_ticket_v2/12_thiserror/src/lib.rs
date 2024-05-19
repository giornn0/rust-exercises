// TODO: Implement the `Error` trait for `TicketNewError` using `thiserror`.
//   We've changed the enum variants to be more specific, thus removing the need for storing
//   a `String` field into each variant.
//   You'll also have to add `thiserror` as a dependency in the `Cargo.toml` file.

use thiserror::Error;

#[derive(Error, Debug)]
enum TicketNewError {
    #[error("{0} cannot be empty")]
    TitleCannotBeEmpty(String),
    #[error("{0} cannot be longer than {1} characters")]
    TitleTooLong(String, u32),
    #[error("{0} cannot be empty")]
    DescriptionCannotBeEmpty(String),
    #[error("{0} cannot be longer than {1} characters")]
    DescriptionTooLong(String, u32),
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty("Title".to_owned()));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong("Title".to_owned(), 50));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty(
                "Description".to_owned(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong(
                "Description".to_owned(),
                500,
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn title_cannot_be_empty() {
        let err = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be empty");
    }

    #[test]
    fn description_cannot_be_empty() {
        let err = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Description cannot be empty");
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let err = Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be longer than 50 characters");
    }

    #[test]
    fn description_cannot_be_too_long() {
        let err = Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Description cannot be longer than 500 characters"
        );
    }
}
