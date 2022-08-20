// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{PhotoSize, File, MaskPosition};
use serde::{Deserialize, Serialize};


/// This object represents a sticker.
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Type of the sticker, currently one of "regular", "mask", "custom_emoji". The type of the sticker is independent from its format, which is determined by the fields is_animated and is_video.
    pub r#type: String,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// True, if the sticker is animated
    pub is_animated: bool,
    /// True, if the sticker is a video sticker
    pub is_video: bool,
    /// Optional. Sticker thumbnail in the .WEBP or .JPG format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Optional. Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Optional. Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    /// Optional. For premium regular stickers, premium animation for the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,
    /// Optional. For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// Optional. For custom emoji stickers, unique identifier of the custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// Optional. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}