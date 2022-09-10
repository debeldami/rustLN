// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn get_perimeter(&self);
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

struct Square {
    l: f64,
}

impl Perimeter for Triangle {
    fn get_perimeter(&self) {
        let area = self.a + self.b + self.c;

        println!("Perimeter = {}", area)
    }
}

impl Perimeter for Square {
    fn get_perimeter(&self) {
        let area = 4.0 * self.l;

        println!("Perimeter = {}", area)
    }
}

fn get_perimeter(polygon: impl Perimeter) {
    polygon.get_perimeter();
}

fn main() {
    let triangle = Triangle {
        a: 2.3,
        b: 1.2,
        c: 8.1,
    };

    let square = Square { l: 5.8 };

    get_perimeter(triangle);
    get_perimeter(square);
}
