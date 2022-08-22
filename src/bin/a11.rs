// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    id: i32,
}

fn display_quantity (grocery: &Grocery){
    println!("quantity of the item is {}", grocery.quantity);
}

fn display_id (grocery: Grocery){
    println!("quantity of the item is {}", grocery.id);
}


fn main() {
    let groc = Grocery{
        quantity: 4,
        id: 6,
    };

    display_quantity(&groc);
    display_id(groc);
}