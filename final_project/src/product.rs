use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
#[derive(Debug, PartialOrd, Clone)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f32,
    pub quantity: f32,
}

impl Product {
    pub fn new(name: String, description: String, quantity: f32, price: f32) -> Self {
        Self {
            name,
            description,
            price,
            quantity,
        }
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.name > other.name {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^24}|{:^24}|{:^24}|{:^24}|{:^24}",
            self.name,
            self.quantity,
            self.price,
            self.description,
            self.price * self.quantity
        )
    }
}

impl Eq for Product {}
