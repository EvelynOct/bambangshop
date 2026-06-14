use std::thread;

use bambangshop::{Result, compose_error_response, APP_CONFIG};
use rocket::http::Status;

use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = product_type_upper.as_str();

        let subscriber_result: Subscriber =
            SubscriberRepository::add(product_type_str, subscriber);

        Ok(subscriber_result)
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = product_type_upper.as_str();

        let result: Option<Subscriber> =
            SubscriberRepository::delete(product_type_str, url);

        if result.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Subscriber not found."),
            ));
        }

        Ok(result.unwrap())
    }

    pub fn notify(product_type: &str, status: &str, product: Product) {
        let mut payload = Notification {
            product_title: product.title.clone(),
            product_type: String::from(product_type),
            product_url: format!(
                "{}/product/{}",
                APP_CONFIG.get_instance_root_url(),
                product.id
            ),
            subscriber_name: String::from(""),
            status: String::from(status),
        };

        let subscribers: Vec<Subscriber> =
            SubscriberRepository::list_all(product_type);

        for subscriber in subscribers {
            payload.subscriber_name = subscriber.clone().name;

            let subscriber_clone = subscriber.clone();
            let payload_clone = payload.clone();

            thread::spawn(move || {
                subscriber_clone.update(payload_clone);
            });
        }
    }
}