// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coordinate() -> (i32, i32){
    (4, 6)
}

fn main() {
    let (x,y) = coordinate();

    if x > 5 {
        println!("x cordinate is greater than 5");
    }else{
        println!("x cordinate is less or equal to 5");
    }

    if y > 5 {
        println!("y cordinate is greater than 5");
    }else{
        println!("y cordinate is less or equal to 5");
    }
}