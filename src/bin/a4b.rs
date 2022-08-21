// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively



fn main() {
    let check_string = 1;

    // * Use a match expression to determine which message to display
    // * Use a variable set to any integer
    match check_string {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"), 
        _ => println!("others") // * Use an underscore (_) to match on any value
    }
}