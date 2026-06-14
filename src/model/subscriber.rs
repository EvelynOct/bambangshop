use rocket::serde::{Deserialize, Serialize};

use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    #[tokio::main]
    pub async fn update(&self, payload: Notification) {
        let client = reqwest::Client::new();

        client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .ok();

        println!(
            "Sent {} notification of {} to {}",
            payload.status,
            payload.product_type,
            self.url
        );
    }
}