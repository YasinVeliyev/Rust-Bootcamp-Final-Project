use crate::product::*;
pub struct Inventory {
    pub store: Vec<Product>,
}

impl Inventory {
    pub fn add(&mut self, product: Product) {
        if let Ok(index) = self.store.binary_search(&product) {
            let ref mut pr = self.store[index];
            pr.quantity += product.quantity;
            pr.price = ((pr.quantity - 1.) * pr.price + product.price) / pr.quantity;
        } else {
            self.store.push(product)
        }
    }

    pub fn delete(&mut self, name: String, quantity: f32) -> Result<(), String> {
        if quantity <= 0. {
            return Err("Out-of-Range Quantity:You insert zero or negative value".to_owned());
        }
        if let Ok(index) = self.store.binary_search_by(|p: &Product| p.name.cmp(&name)) {
            let ref mut pr = self.store[index];
            if pr.quantity >= quantity {
                pr.quantity -= quantity;
                return Ok(());
            } else {
                return Err("Insufficient Stock".to_string());
            }
        } else {
            return Err("Item Not Found".to_owned());
        }
    }
}
