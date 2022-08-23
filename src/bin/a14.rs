// Topic: Strings
//
// Requirements:
// * Print out the name a&nd favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person{
    age: i32,
    name: String,
    favorite_color: String,
}

fn print_name_color( name: &str, favorite_color : &str){
    println!("name of person is {} and favorite color is {}", name, favorite_color)
}

fn main() {
    let person_vec = vec![
        Person{
            age: 12,
            name: "servant".to_string(),
            favorite_color: "brown".to_string()
        },
        Person{
            age: 16,
            name: "king".to_string(),
            favorite_color: "pink".to_string()
        },
        Person{
            age: 19,
            name: "queen".to_string(),
            favorite_color: "purple".to_string()
        },
    ];

    for person in person_vec {
        println!("age is {}", person.age);
        print_name_color(&person.name, &person.favorite_color)
    }
}