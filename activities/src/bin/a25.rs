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

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.a + self.b + self.c
    }
}

struct Square {
    side: i32,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side * 4
    }
}

trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

fn print_perimeter(shape: impl Perimeter) {
    println!("The perimeter is: {}", shape.calculate_perimeter())
}

fn main() {
    print_perimeter(Square { side: 4 });
    print_perimeter(Triangle { a: 3, b: 4, c: 4 })
}
