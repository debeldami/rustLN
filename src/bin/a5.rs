// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut loop_breaker = 1;
    loop{
        if loop_breaker == 4 {
            break;
        }
        println!("current value of loop breaker is {}", loop_breaker);
        loop_breaker += 1;
    }
}