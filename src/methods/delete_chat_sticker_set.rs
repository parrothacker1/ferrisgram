// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    /// <https://core.telegram.org/bots/api#deletechatstickerset>
    pub fn delete_chat_sticker_set(&self, chat_id: i64) -> DeleteChatStickerSetBuilder {
        DeleteChatStickerSetBuilder::new(self, chat_id)
    }
}

#[derive(Serialize)]
pub struct DeleteChatStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
}

impl<'a> DeleteChatStickerSetBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64) -> Self {
        Self { bot, chat_id }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("deleteChatStickerSet", Some(&form)).await
    }
}
