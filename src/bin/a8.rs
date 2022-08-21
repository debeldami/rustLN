// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavor {
    Venilla,
    Strawberry,
}

struct Drink {
    flavor: Flavor,
    ounce: i32, 
}

fn check_drink (drink: Drink){
    println!("drink ounce {}", drink.ounce);

    match  drink.flavor {
        Flavor::Strawberry =>     println!("drink flavor is {}", "Strawberry"),
        Flavor::Venilla =>     println!("drink flavor is {}", "Venilla"),
    }
}

fn main() {
    let strawberry_ice = Drink { flavor: Flavor::Strawberry, ounce: 2 };
    check_drink(strawberry_ice);
}