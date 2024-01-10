// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::ChatAdministratorRights;
use crate::Bot;

impl Bot {
    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns True on success.
    /// <https://core.telegram.org/bots/api#setmydefaultadministratorrights>
    pub fn set_my_default_administrator_rights(&self) -> SetMyDefaultAdministratorRightsBuilder {
        SetMyDefaultAdministratorRightsBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct SetMyDefaultAdministratorRightsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    /// Pass True to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

impl<'a> SetMyDefaultAdministratorRightsBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            rights: None,
            for_channels: None,
        }
    }

    pub fn rights(mut self, rights: ChatAdministratorRights) -> Self {
        self.rights = Some(rights);
        self
    }

    pub fn for_channels(mut self, for_channels: bool) -> Self {
        self.for_channels = Some(for_channels);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get("setMyDefaultAdministratorRights", Some(&form))
            .await
    }
}
