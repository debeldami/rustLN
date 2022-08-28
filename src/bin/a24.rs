// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data: Vec<i32> = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|val| val * 3)
        .filter(|val| val > &10)
        .collect();

    for val in data {
        println!("{}", val);
    }
}
