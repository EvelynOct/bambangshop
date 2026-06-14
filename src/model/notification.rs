use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    pub product_id: String,
    pub product_name: String,
    pub product_type: String,
    pub notification_type: String,
}