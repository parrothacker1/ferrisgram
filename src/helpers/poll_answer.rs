// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::PollAnswer;

impl PollAnswer {
    /// This function creates an empty struct for the object PollAnswer.
    pub fn new() -> Self {
        Self {
            poll_id: "".to_string(),
            voter_chat: None,
            user: None,
            option_ids: Vec::new(),
        }
    }
}
impl Default for PollAnswer {
    fn default() -> Self {
        Self::new()
    }
}
