// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::input_file::InputFile;
use crate::types::Message;
use crate::types::{InlineKeyboardMarkup, ReplyParameters};
use crate::Bot;
use std::collections::HashMap;

impl Bot {
    /// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned.
    /// <https://core.telegram.org/bots/api#sendsticker>
    pub fn send_sticker<F: InputFile>(&self, chat_id: i64, sticker: F) -> SendStickerBuilder<F> {
        SendStickerBuilder::new(self, chat_id, sticker)
    }
}

#[derive(Serialize)]
pub struct SendStickerBuilder<'a, F: InputFile> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip)]
    data: HashMap<&'a str, F>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Emoji associated with the sticker; only for just uploaded stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
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

impl<'a, F: InputFile> SendStickerBuilder<'a, F> {
    pub fn new(bot: &'a Bot, chat_id: i64, sticker: F) -> Self {
        let mut data = HashMap::new();
        data.insert("sticker", sticker);
        Self {
            bot,
            data,
            chat_id,
            message_thread_id: None,
            emoji: None,
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

    pub fn sticker(mut self, sticker: F) -> Self {
        self.data.insert("sticker", sticker);
        self
    }

    pub fn emoji(mut self, emoji: String) -> Self {
        self.emoji = Some(emoji);
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
        self.bot
            .post("sendSticker", Some(&form), Some(self.data))
            .await
    }
}
