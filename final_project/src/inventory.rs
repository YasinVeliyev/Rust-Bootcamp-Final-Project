use crate::product;
use crate::product::*;
use crate::report::*;
use crate::transaction::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::io;
pub struct Inventory {
    pub store: HashMap<String, Product>,
    pub transactions: Vec<Transaction>,
}

impl Report for Inventory {
    fn report(&self) {
        // let mut total = 0.;
        println!("{}", "-".repeat(120));
        println!(
            "|{:^24}|{:^24}|{:^24}|{:^24}|{:^24}|",
            "Name", "Quantity", "Price", "Total", "Description",
        );
        println!("{}", "-".repeat(126));
        self.store.iter().for_each(|(name, product)| {
            println!("{}", product);
            println!("{}", "-".repeat(126));
        })
    }
}

impl Inventory {
    pub fn new(products: HashMap<String, Product>) -> Self {
        Self {
            store: products,
            transactions: Vec::new(),
        }
    }

    pub fn add(&mut self, product: Product) {
        self.store
            .entry(product.name.clone())
            .and_modify(|prod| {
                prod.quantity += product.quantity;
                prod.price = (prod.price * prod.quantity + product.price * product.quantity)
                    / (prod.quantity + product.quantity);
            })
            .or_insert(product);
    }

    pub fn delete(&mut self, name: String) -> Result<(), String> {
        match self.store.remove(&name) {
            Some(_) => Ok(()),
            None => return Err("Item Not Found".to_owned()),
        }
    }

    fn ask() -> Product {
        let mut name = String::new();
        let mut description = String::new();
        let mut price = 0.;
        let mut quantity = 0.;
        let mut product = loop {
            if name.is_empty() {
                println!("Enter product name");
                io::stdin().read_line(&mut name).unwrap();
                name = name.trim().to_owned();
            }

            if price == 0. {
                println!("Enter product price");
                let mut pirce_str = String::new();
                io::stdin().read_line(&mut pirce_str).unwrap();
                match pirce_str.trim().parse::<f32>() {
                    Ok(pr) => {
                        if pr <= 0. {
                            println!("Enter a positive float number");
                            continue;
                        }
                        price = pr
                    }
                    Err(_) => {
                        println!("Enter a float number");
                        continue;
                    }
                }
            }
            if description.is_empty() {
                println!("Enter product description");
                io::stdin().read_line(&mut description).unwrap();
                description = description.trim().to_owned();
            }

            if quantity == 0. {
                println!("Enter product quantity");
                let mut quantity_str = String::new();
                io::stdin().read_line(&mut quantity_str).unwrap();
                match quantity_str.trim().parse::<f32>() {
                    Ok(qt) => {
                        if qt <= 0. {
                            println!("Enter a positive float number");
                            continue;
                        }
                        quantity = qt
                    }
                    Err(_) => {
                        println!("Enter a float number");
                        continue;
                    }
                }
            }
            break Product::new(name.to_owned(), description.to_owned(), quantity, price);
        };
        product
    }
    pub fn edit(&mut self, name: &str) -> Result<(), String> {
        match self.store.get_mut(name) {
            Some(prod) => {
                *prod = Self::ask();
                return Ok(());
            }
            None => return Err("Product not found".to_owned()),
        }
    }

    pub fn sell(&mut self, name: &str, quantity: f32, price: f32) -> Result<(), String> {
        match self.store.get_mut(name) {
            Some(product) => {
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
            None => Err("Product not found".to_owned()),
        }
    }

    pub fn buy(&mut self) -> Result<(), String> {
        let product = Self::ask();
        self.add(product.clone());
        self.transactions
            .push(Transaction::new(product, Action::Buy));
        Ok(())
    }
}
