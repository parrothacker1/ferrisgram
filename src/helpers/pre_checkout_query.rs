// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::PreCheckoutQuery;
use crate::types::User;

impl PreCheckoutQuery {
    /// This function creates an empty struct for the object PreCheckoutQuery.
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            from: User::new(),
            currency: "".to_string(),
            total_amount: 0,
            invoice_payload: "".to_string(),
            shipping_option_id: None,
            order_info: None,
        }
    }
}
impl Default for PreCheckoutQuery {
    fn default() -> Self {
        Self::new()
    }
}
