use crate::{product::*, report::Report};
use chrono::prelude::*;

pub struct Transaction {
    pub date: String,
    pub action: Action,
    pub product: Product,
}

pub enum Action {
    Sell,
    Buy,
}

impl Transaction {
    pub fn new(product: Product, action: Action) -> Self {
        let date = Local::now();
        Self {
            date: date.format("%d.%m.%Y %H:%M:%S").to_string(),
            action,
            product,
        }
    }
}
