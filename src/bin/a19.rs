use std::collections::HashMap;

// Topic: HashMap

//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

fn total_item (store: &HashMap<&str, i32>) {
    let mut total_item = 0;
    for amount in store.values() {
        total_item += amount;
    }

    println!("total item is {}", total_item);
}

fn is_available (store: &HashMap<&str, i32>, item: &str) {
    
    match store.get(item) {
        Some(val) => {
            match val {
                0 => println!("out of stock"),
                _ => println!("availble") 
            }
        }
        None => println!("item not listed"),
    }
}

fn main() {

    let mut store = HashMap::new();
    store.insert("Couches", 0);
    store.insert("Tables", 2);
    store.insert("Beds", 3);
    store.insert("Chairs", 5);

    total_item(&store);
    is_available(&store, "Chairs")
}