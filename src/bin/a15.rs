// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket{
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32),
}

fn main() {
    let ticket_list = vec![
    Ticket::Backstage(20, "Mike".to_string()), 
    Ticket::Vip(40, "John".to_string()), 
    Ticket::Standard(10)
    ];

    for ticket in ticket_list {
        match  ticket {
            Ticket::Backstage(price,name ) => println!("price {} and name {}", price,name ),
            Ticket::Vip(price,name ) => println!("price {} and name {}",price, name ),
            Ticket::Standard(price) => println!("price {} for standard",price ),
        }
    }
}
