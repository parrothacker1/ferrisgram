// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::MenuButton;
use crate::Bot;

impl Bot {
    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success.
    /// <https://core.telegram.org/bots/api#getchatmenubutton>
    pub fn get_chat_menu_button(&self) -> GetChatMenuButtonBuilder {
        GetChatMenuButtonBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct GetChatMenuButtonBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

impl<'a> GetChatMenuButtonBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self { bot, chat_id: None }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub async fn send(self) -> Result<MenuButton> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("getChatMenuButton", Some(&form)).await
    }
}
