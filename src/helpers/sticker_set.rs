// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::StickerSet;

impl StickerSet {
    /// This function creates an empty struct for the object StickerSet.
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            title: "".to_string(),
            sticker_type: "".to_string(),
            is_animated: false,
            is_video: false,
            stickers: Vec::new(),
            thumb: None,
        }
    }
}
impl Default for StickerSet {
    fn default() -> Self {
        Self::new()
    }
}