// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InputMediaPhoto;

impl InputMediaPhoto {
    /// This function creates an empty struct for the object InputMediaPhoto.
    pub fn new() -> Self {
        Self {
            media: "".to_string(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
        }
    }
}
impl Default for InputMediaPhoto {
    fn default() -> Self {
        Self::new()
    }
}
