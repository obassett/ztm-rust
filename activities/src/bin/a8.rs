// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum DrinkFlavour {
    Cola,
    Raspberry,
    Lemonade,
}

struct Drink {
    name: String,
    flavour: DrinkFlavour,
    fluid_oz: i32,
}

fn print_drink_info(my_drink: Drink) {
    let flavour_text = match my_drink.flavour {
        DrinkFlavour::Cola => "Cola",
        DrinkFlavour::Lemonade => "Lemonade",
        DrinkFlavour::Raspberry => "Raspberry",
    };
    println!(
        "Your {:?} is  {:?} flavour, and has a size of {:?} fl. oz",
        my_drink.name, flavour_text, my_drink.fluid_oz
    );
}

fn main() {
    let coke = Drink {
        name: "Coca-Cola".to_string(),
        flavour: DrinkFlavour::Cola,
        fluid_oz: 20,
    };

    print_drink_info(coke);
}
