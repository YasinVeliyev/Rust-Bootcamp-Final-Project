use final_project::inventory::*;
use final_project::product::Product;
use final_project::report::Report;
use serde_json;
use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("./data.json").expect("Unable to read file");
    let res: Vec<Product> = serde_json::from_str(&data).unwrap();
    let store: HashMap<String, Product> = HashMap::from_iter(
        res.iter()
            .map(|product| (product.name.clone(), product.clone())),
    );
    let inventory = Inventory::new(store);
    let mut product = Product::new("Alma".to_owned(), "red".to_owned(), 5., 1.);
    inventory.report();
}
