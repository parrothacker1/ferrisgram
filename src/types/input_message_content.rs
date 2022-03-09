// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{
    InputContactMessageContent, InputInvoiceMessageContent, InputLocationMessageContent,
    InputTextMessageContent, InputVenueMessageContent,
};
use serde::{Deserialize, Serialize};

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
/// - InputTextMessageContent
/// - InputLocationMessageContent
/// - InputVenueMessageContent
/// - InputContactMessageContent
/// - InputInvoiceMessageContent
/// <https://core.telegram.org/bots/api#inputmessagecontent>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
    InputInvoiceMessageContent(InputInvoiceMessageContent),
}
