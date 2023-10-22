use crate::product::{self, *};
use std::{cmp::Ordering, io};
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

    pub fn delete(&mut self, name: String) -> Result<(), String> {
        let length = self.store.len();
        self.store.retain(|p: &Product| !p.name.cmp(&name).is_eq());
        if length == self.store.len() {
            return Err("Item Not Found".to_owned());
        } else {
            Ok(())
        }
    }

    pub fn edit(&mut self, name: String) -> Result<(), String> {
        let mut new_name = String::new();
        let mut description = String::new();
        let mut price = String::new();
        let mut quantity = String::new();
        let index;

        match self.find(name) {
            Ok(i) => index = i,
            Err(_) => return Err("Item Not Found".to_owned()),
        }

        loop {
            println!(
                "Current name is {} For change name enter new name",
                self.store[index].name
            );
            io::stdin().read_line(&mut new_name).unwrap();
            if new_name.trim() != "" {
                self.store[index].name = new_name.trim().to_owned();
            }

            println!(
                "Current price is {} for change price enter a float",
                self.store[index].price
            );

            io::stdin().read_line(&mut price).unwrap();
            if price.trim() != "" {
                match price.trim().parse::<f32>() {
                    Ok(price) => self.store[index].price = price,
                    Err(_) => {
                        println!("Enter a float number");
                        continue;
                    }
                }
            }

            println!(
                "Current description is {} For change description enter text",
                self.store[index].description
            );
            io::stdin().read_line(&mut description).unwrap();
            if description.trim() != "" {
                self.store[index].description = description.trim().to_owned();
            }

            println!(
                "Current quantity is {} For change quantity enter a float",
                self.store[index].quantity
            );
            io::stdin().read_line(&mut quantity).unwrap();

            if quantity.trim() != "" {
                match quantity.trim().parse::<f32>() {
                    Ok(quantity) => self.store[index].quantity = quantity,
                    Err(_) => {
                        println!("Enter a float number");
                        continue;
                    }
                }
            }
            break;
        }

        Ok(())
    }

    fn find(&mut self, name: String) -> Result<usize, ()> {
        if let Ok(index) = self.store.binary_search_by(|p: &Product| p.name.cmp(&name)) {
            return Ok(index);
        } else {
            Err(())
        }
    }
}
