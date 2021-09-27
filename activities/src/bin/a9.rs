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
use rand::Rng;

fn get_coordinates() -> (i32, i32) {
    let mut rng = rand::thread_rng();

    let x: i32 = rng.gen_range(0..10);
    let y: i32 = rng.gen_range(0..10);
    (x, y)
}

fn main() {
    let (x, y) = get_coordinates();

    if y > 5 {
        println! {"The co-ordinate are- x:{:?}  y:{:?}  - y is greater than 5",x ,y}
    } else if y < 5 {
        println! {"The co-ordinate are- x:{:?}  y:{:?}  - y is less than 5",x ,y}
    } else {
        println! {"The co-ordinate are- x:{:?}  y:{:?}  - y is equal to 5",x ,y}
    }
}
