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

enum BoxColour {
    Red,
    Blue,
    Green,
}

struct Box {
    height: i32,
    width: i32,
    length: i32,
    color: BoxColour,
    weight: i32,
}

impl Box {
    fn new_standard_box() -> Self {
        Self {
            height: 10,
            width: 10,
            length: 20,
            color: BoxColour::Green,
            weight: 5,
        }
    }

    fn get_box_color(&self) -> String {
        match self.color {
            BoxColour::Blue => String::from("Blue"),
            BoxColour::Green => String::from("Green"),
            BoxColour::Red => String::from("Red"),
        }
    }

    fn info(&self) {
        println!(
            "This box is {:?}cm * {:?}cm * {:?}cm and weigh's {:?} grams and is the colour {:?}",
            self.height,
            self.width,
            self.length,
            self.weight,
            self.get_box_color()
        )
    }
}

fn main() {
    let my_box = Box::new_standard_box();

    my_box.info();
}
