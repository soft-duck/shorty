use thiserror::Error;
use yew::AttrValue;

use crate::components::message_box::Message;

#[derive(Error, Clone, Debug)]
pub enum FormError {
    // TODO shorten button could be disabled to prevent this
    #[error("You need to provide a link to shorten first.")]
    LinkInputEmpty,
    #[error("Link too long. This instance is configured to only allow links with up to {max_length} characters.")]
    ExceededMaxLinkLength { link: String, max_length: usize },
    #[error("Custom Id too long. This instance is configured to only allow Ids with up to {max_length} characters.")]
    ExceededMaxIdLength { id: String, max_length: usize },
    #[error("{number} is not a valid number. Please input a valid one.")]
    ParseNumberFailure { number: String },
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Maximum json side exceeded")]
    JsonSizeExceeded,
    #[error("Request to backend unsuccessful: {error}")]
    // TODO better name
    UnsuccessfulRequest {
        error: reqwest::Error,
    },
    // TODO make error more specific by documenting the backend error possibilities
    #[error("Json malformed")]
    Backend400,
    #[error("Id '{id}' already in use")]
    IdInUse {
        id: String,
    },
}

impl Into<Message> for RequestError {
    fn into(self) -> Message {
        Message::Error(AttrValue::from(self.to_string()))
    }
}

impl Into<Message> for FormError {
    fn into(self) -> Message {
        Message::Error(AttrValue::from(self.to_string()))
    }
}
