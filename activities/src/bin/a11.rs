// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn print_item_quantity(item: &GroceryItem) {
    println!("Qty: {:?}", item.quantity)
}

fn print_item_id(item: &GroceryItem) {
    println!("ID: {:?}", item.id)
}

fn main() {
    let item = GroceryItem {
        id: 7,
        quantity: 100,
    };

    print_item_quantity(&item);
    print_item_id(&item);
}
