use crate::product::*;
use crate::report::*;
use crate::transaction;
use crate::transaction::*;
use crate::users::Role;
use crate::users::User;
use std::collections::HashMap;

use std::io;
pub struct Inventory {
    pub store: HashMap<String, Product>,
    pub transactions: Vec<Transaction>,
}

impl Report for Inventory {
    fn report(&self) {
        // let mut total = 0.;
        println!("{}", "-".repeat(78));
        println!("|{}|", " ".repeat(76));
        println!(
            "|{:^24}|{:^8}|{:^8}|{:^8}|{:^24}|",
            "Name", "Quantity", "Price", "Total", "Description",
        );
        println!("|{}|", " ".repeat(76));
        println!("{}", "-".repeat(78));
        self.store.iter().for_each(|(_, product)| {
            println!("{}", product);
            println!("{}", "-".repeat(78));
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
            .entry(product.name.clone().to_ascii_lowercase())
            .and_modify(|prod| {
                prod.quantity += product.quantity;
                prod.price = (prod.price * prod.quantity + product.price * product.quantity)
                    / (prod.quantity + product.quantity);
                dbg!(&prod);
            })
            .or_insert(product);
    }

    pub fn delete(&mut self, name: &str) -> Result<(), String> {
        match self.store.remove(&name.to_ascii_lowercase()) {
            Some(_) => Ok(()),
            None => return Err("Item Not Found".to_owned()),
        }
    }

    fn ask() -> Product {
        let mut name = String::new();
        let mut description = String::new();
        let mut price = 0.;
        let mut quantity = 0.;
        let product = loop {
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
        match self.store.get_mut(&name.to_ascii_lowercase()) {
            Some(prod) => {
                *prod = Self::ask();
                return Ok(());
            }
            None => return Err("Product not found".to_owned()),
        }
    }

    pub fn sell(&mut self, prod: Product) -> Result<(), String> {
        match self.store.get_mut(&prod.name.to_ascii_lowercase()) {
            Some(product) => {
                if product.quantity < prod.quantity {
                    return Err("Insufficient Stock".to_owned());
                } else {
                    product.quantity -= prod.quantity;

                    let transaction = Transaction::new(prod, Action::Sell);
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

    pub fn login(&mut self, user: User) {
        if user.role == Role::InventoryManager {
            loop {
                println!("1. Report \n2. Add Product\n3. Edit Product\n4. Delete Product\n5. Buy Product\n6. Sell Product\n7. Transactions\n8. Exit");
                let mut selection = String::new();
                io::stdin().read_line(&mut selection).unwrap();
                match selection.trim() {
                    "1" => self.report(),
                    "2" => {
                        let product = Self::ask();
                        self.add(product)
                    }
                    "3" => {
                        println!("Enter Product name for edit");
                        let mut name = String::new();
                        io::stdin().read_line(&mut name).unwrap();
                        match self.edit(name.trim()) {
                            Ok(_) => continue,
                            Err(err) => {
                                println!("{}", err);
                            }
                        };
                    }
                    "4" => {
                        println!("Enter Product name for delete");
                        let mut name = String::new();
                        io::stdin().read_line(&mut name).unwrap();
                        match self.delete(name.trim()) {
                            Ok(_) => continue,
                            Err(err) => {
                                println!("{}", err);
                            }
                        };
                    }
                    "5" => match self.buy() {
                        Ok(_) => continue,
                        Err(err) => println!("{}", err),
                    },
                    "6" => {
                        let product = Self::ask();
                        match self.sell(product) {
                            Ok(_) => continue,
                            Err(err) => println!("{}", err),
                        }
                    }
                    "7" => {
                        println!("{}", "-".repeat(108));
                        println!(
                            "|{:^20}|{:^20}|{:^8}|{:^8}|{:^8}|{:^24}|{:^12}|",
                            "Date", "Name", "Quantity", "Price", "Total", "Description", "Action",
                        );
                        println!("{}", "-".repeat(108));
                        for transaction in &self.transactions {
                            match transaction.action {
                                Action::Sell => println!(
                                    "|{:^20}{}{:^6}|{:^6}|",
                                    transaction.date, transaction.product, " ", "Sell"
                                ),
                                Action::Buy => {
                                    println!(
                                        "|{:^20}{}{:^6}|{:^6}|",
                                        transaction.date, transaction.product, "Buy", " "
                                    )
                                }
                            }
                        }
                        println!("{}\n", "-".repeat(108));
                    }
                    "8" => break,
                    _ => {
                        println!("Wrong number");
                    }
                }
            }
        }
    }
}



