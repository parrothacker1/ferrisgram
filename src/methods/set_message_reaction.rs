// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::ReactionType;
use crate::Bot;

impl Bot {
    /// Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns True on success.
    /// <https://core.telegram.org/bots/api#setmessagereaction>
    pub fn set_message_reaction(&self, chat_id: i64, message_id: i64) -> SetMessageReactionBuilder {
        SetMessageReactionBuilder::new(self, chat_id, message_id)
    }
}

#[derive(Serialize)]
pub struct SetMessageReactionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    pub message_id: i64,
    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Vec<ReactionType>>,
    /// Pass True to set the reaction with a big animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

impl<'a> SetMessageReactionBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, message_id: i64) -> Self {
        Self {
            bot,
            chat_id,
            message_id,
            reaction: None,
            is_big: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = message_id;
        self
    }

    pub fn reaction(mut self, reaction: Vec<ReactionType>) -> Self {
        self.reaction = Some(reaction);
        self
    }

    pub fn is_big(mut self, is_big: bool) -> Self {
        self.is_big = Some(is_big);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("setMessageReaction", Some(&form)).await
    }
}
