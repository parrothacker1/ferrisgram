// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ForumTopic;

impl ForumTopic {
    /// This function creates an empty struct for the object ForumTopic.
    pub fn new() -> Self {
        Self {
            message_thread_id: 0,
            name: "".to_string(),
            icon_color: 0,
            icon_custom_emoji_id: None,
        }
    }
}
impl Default for ForumTopic {
    fn default() -> Self {
        Self::new()
    }
}
