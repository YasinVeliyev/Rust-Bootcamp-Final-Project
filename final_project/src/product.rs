use std::cmp::Ordering;

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

impl Eq for Product {}
