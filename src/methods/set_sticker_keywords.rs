// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    /// <https://core.telegram.org/bots/api#setstickerkeywords>
    pub fn set_sticker_keywords(&self, sticker: String) -> SetStickerKeywordsBuilder {
        SetStickerKeywordsBuilder::new(self, sticker)
    }
}

#[derive(Serialize)]
pub struct SetStickerKeywordsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

impl<'a> SetStickerKeywordsBuilder<'a> {
    pub fn new(bot: &'a Bot, sticker: String) -> Self {
        Self {
            bot,
            sticker,
            keywords: None,
        }
    }

    pub fn sticker(mut self, sticker: String) -> Self {
        self.sticker = sticker;
        self
    }

    pub fn keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = Some(keywords);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get::<bool>("setStickerKeywords", Some(&form))
            .await
    }
}
