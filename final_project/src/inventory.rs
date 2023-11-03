use crate::product::{self, *};
use crate::transaction::{self, *};

use std::{cmp::Ordering, io};
pub struct Inventory {
    pub store: Vec<Product>,
    pub transactions: Vec<Transaction>,
}

impl Inventory {
    pub fn add(&mut self, product: Product) {
        if let Ok(index) = self.store.binary_search(&product) {
            let ref mut pr = self.store[index];
            pr.quantity += product.quantity;
            pr.price = product.price;
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

    pub fn edit(&mut self, name: &str) -> Result<(), String> {
        let mut new_name = String::new();
        let mut description = String::new();
        let mut price = String::new();
        let mut quantity = String::new();
        let index;

        match self.find(name) {
            Ok(i) => index = i,
            Err(_) => return Err("Product Not Found".to_owned()),
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

    fn find(&mut self, name: &str) -> Result<usize, ()> {
        if let Ok(index) = self
            .store
            .binary_search_by(|p: &Product| p.name.cmp(&name.to_owned()))
        {
            return Ok(index);
        } else {
            Err(())
        }
    }

    pub fn sell(&mut self, name: &str, quantity: f32, price: f32) -> Result<(), String> {
        for product in &mut self.store {
            if product.name == name {
                if product.quantity < quantity {
                    return Err("Insufficient Stock".to_owned());
                } else {
                    let mut sell_product = product.clone();
                    product.quantity -= quantity;
                    sell_product.quantity = quantity;
                    sell_product.price = price;
                    let transaction = Transaction::new(sell_product, Action::Sell);
                    self.transactions.push(transaction);
                    return Ok(());
                }
            }
        }

        Err("Product not found".to_owned())
    }

    pub fn buy(&mut self) -> Result<(), String> {
        let mut name = String::new();
        let mut description = String::new();
        let mut price = 0.;
        let mut quantity = 0.;

        loop {
            println!("Enter product name");

            if name.is_empty() {
                io::stdin().read_line(&mut name).unwrap();
                name = name.trim().to_owned();
            }
            if description.is_empty() {
                io::stdin().read_line(&mut description).unwrap();
                description = description.trim().to_owned();
            }
            if price == 0. {
                let mut price_str = String::new();
                io::stdin().read_line(&mut price_str).unwrap();
                match price_str.trim().parse::<f32>() {
                    Ok(p) => price = p,

                    Err(_) => {
                        println!("Enter a float number");
                        continue;
                    }
                }
            }
            if quantity == 0. {
                let mut quantity_str = String::new();
                io::stdin().read_line(&mut quantity_str).unwrap();
                match quantity_str.trim().parse::<f32>() {
                    Ok(q) => quantity = q,

                    Err(_) => {
                        println!("Enter a float number");
                        continue;
                    }
                }
            }

            break;
        }

        let product = Product::new(name, description, quantity, price);
        self.add(product);
        Ok(())
    }
}
