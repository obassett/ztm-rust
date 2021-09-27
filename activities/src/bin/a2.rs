// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// * Use a function to add two numbers together

fn sum_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// * Use a function to display the result
fn display_value(value: String) {
    print!("{:?}", value);
}

fn main() {
    let my_value = sum_two_numbers(5, 3);

    display_value(my_value.to_string());
}
