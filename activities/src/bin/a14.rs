// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    favourite_color: String,
}

impl Person {
    fn print(&self) {
        print_it(&self.name);
        print_it(&self.favourite_color);
    }
}

fn print_it(text: &str) {
    println!("{:?}", text);
}

fn main() {
    let party = vec![
        Person {
            name: String::from("John"),
            age: 17,
            favourite_color: String::from("blue"),
        },
        Person {
            name: String::from("Alice"),
            age: 13,
            favourite_color: String::from("orange"),
        },
        Person {
            name: String::from("Hana"),
            age: 3,
            favourite_color: String::from("pink"),
        },
        Person {
            name: String::from("Bob"),
            age: 7,
            favourite_color: String::from("yellow"),
        },
    ];

    for kid in party {
        if kid.age <= 10 {
            kid.print();
        }
    }
}
