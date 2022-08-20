// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers

// * Use a function to display the result
fn main(){
    let c = add(3,5);

    // * Use the "{:?}" token in the println macro to display the result
    println!("sum of 3 and 5: {:?}", c)

}

// * Use a function to add two numbers together
fn add(a:i8, b: i8) -> i8 {
    a+b
}