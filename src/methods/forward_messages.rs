// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::MessageId;
use crate::Bot;

impl Bot {
    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of MessageId of the sent messages is returned.
    /// <https://core.telegram.org/bots/api#forwardmessages>
    pub fn forward_messages(
        &self,
        chat_id: i64,
        from_chat_id: i64,
        message_ids: Vec<i64>,
    ) -> ForwardMessagesBuilder {
        ForwardMessagesBuilder::new(self, chat_id, from_chat_id, message_ids)
    }
}

#[derive(Serialize)]
pub struct ForwardMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Unique identifier for the chat where the original messages were sent (or channel username in the format @channelusername)
    pub from_chat_id: i64,
    /// A JSON-serialized list of 1-100 identifiers of messages in the chat from_chat_id to forward. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<i64>,
    /// Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

impl<'a> ForwardMessagesBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, from_chat_id: i64, message_ids: Vec<i64>) -> Self {
        Self {
            bot,
            chat_id,
            message_thread_id: None,
            from_chat_id,
            message_ids,
            disable_notification: None,
            protect_content: None,
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

    pub fn from_chat_id(mut self, from_chat_id: i64) -> Self {
        self.from_chat_id = from_chat_id;
        self
    }

    pub fn message_ids(mut self, message_ids: Vec<i64>) -> Self {
        self.message_ids = message_ids;
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

    pub async fn send(self) -> Result<Vec<MessageId>> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("forwardMessages", Some(&form)).await
    }
}
