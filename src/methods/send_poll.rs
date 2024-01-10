// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::Message;
use crate::types::{InlineKeyboardMarkup, MessageEntity, ReplyParameters};
use crate::Bot;

impl Bot {
    /// Use this method to send a native poll. On success, the sent Message is returned.
    /// <https://core.telegram.org/bots/api#sendpoll>
    pub fn send_poll(
        &self,
        chat_id: i64,
        question: String,
        options: Vec<String>,
    ) -> SendPollBuilder {
        SendPollBuilder::new(self, chat_id, question, options)
    }
}

#[derive(Serialize)]
pub struct SendPollBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Poll question, 1-300 characters
    pub question: String,
    /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    /// True, if the poll needs to be anonymous, defaults to True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type, "quiz" or "regular", defaults to "regular"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll explanation, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    /// Pass True if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> SendPollBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, question: String, options: Vec<String>) -> Self {
        Self {
            bot,
            chat_id,
            message_thread_id: None,
            question,
            options,
            is_anonymous: None,
            r#type: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn question(mut self, question: String) -> Self {
        self.question = question;
        self
    }

    pub fn options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }

    pub fn r#type(mut self, r#type: String) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn allows_multiple_answers(mut self, allows_multiple_answers: bool) -> Self {
        self.allows_multiple_answers = Some(allows_multiple_answers);
        self
    }

    pub fn correct_option_id(mut self, correct_option_id: i64) -> Self {
        self.correct_option_id = Some(correct_option_id);
        self
    }

    pub fn explanation(mut self, explanation: String) -> Self {
        self.explanation = Some(explanation);
        self
    }

    pub fn explanation_parse_mode(mut self, explanation_parse_mode: String) -> Self {
        self.explanation_parse_mode = Some(explanation_parse_mode);
        self
    }

    pub fn explanation_entities(mut self, explanation_entities: Vec<MessageEntity>) -> Self {
        self.explanation_entities = Some(explanation_entities);
        self
    }

    pub fn open_period(mut self, open_period: i64) -> Self {
        self.open_period = Some(open_period);
        self
    }

    pub fn close_date(mut self, close_date: i64) -> Self {
        self.close_date = Some(close_date);
        self
    }

    pub fn is_closed(mut self, is_closed: bool) -> Self {
        self.is_closed = Some(is_closed);
        self
    }

    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn reply_parameters(mut self, reply_parameters: ReplyParameters) -> Self {
        self.reply_parameters = Some(reply_parameters);
        self
    }

    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub async fn send(self) -> Result<Message> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("sendPoll", Some(&form)).await
    }
}
