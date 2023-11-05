use final_project::product::{self, Product};

use final_project::*;

fn main() {
    let mut product = Product::new("Alma".to_owned(), "red".to_owned(), 5., 1.);
    println!("{}", product);
}
