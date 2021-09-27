// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Blue,
    Green,
    Pink,
}

fn print_color(color: Color) {
    let color_string = match color {
        Color::Blue => "blue",
        Color::Green => "green",
        Color::Red => "red",
        Color::Pink => "pink",
    };
    println!("{:?}", color_string);
}

fn main() {
    let my_favourite_color = Color::Pink;

    print_color(my_favourite_color);
}
