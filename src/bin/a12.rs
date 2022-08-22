// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct  Box{
    dimension: Dimension,
    weight: f64,
    color: Color,
}

impl Box {
    fn new (dimension: Dimension, weight: f64, color:Color) -> Self{
        Self { dimension, weight, color }
    }

    fn print(&self){
        self.dimension.print();
        println!("Box weight is {}", self.weight);
        self.color.print();
    }
}

struct  Dimension {
    height: f64,
    width: f64,
    depth: f64
}

impl Dimension {

    fn print(&self){
        println!("width :{:?}", self.width);
        println!("height :{:?}", self.height);
        println!("depth :{:?}", self.depth);
    }
}

enum Color {
    Blue,
    Red,
}


impl Color {

    fn print(&self){
        match  self {
            Color::Blue => println!("color blue"),
            Color::Red => println!("color red"),
        }
    }
}





fn main() {

    let small = Dimension{
        height: 5.5,
        width: 5.5,
        depth: 5.5,
    };

    let color = Color::Blue;


    let box1 = Box::new( small, 5.0, color );

    box1.print();
}