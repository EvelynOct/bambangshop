use std::sync::Mutex;

use crate::model::subscriber::Subscriber;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SUBSCRIBERS: Mutex<Vec<Subscriber>> = Mutex::new(Vec::new());
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(subscriber: Subscriber) {
        let mut subscribers = SUBSCRIBERS.lock().unwrap();
        subscribers.push(subscriber);
    }

    pub fn list_all(product_type: &str) -> Vec<Subscriber> {
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
        }

        return SUBSCRIBERS
            .get(product_type)
            .unwrap()
            .iter()
            .map(|f| f.value().clone())
            .collect();
    }
}