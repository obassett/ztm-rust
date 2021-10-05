// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io::{self, Write};

#
struct Bill {
    payee: String,
    amount: f32,
}

impl Bill {
    fn new(payee: &str, amount: f32) -> Bill {
        Bill {
            payee: payee.to_owned(),
            amount: amount,
        }
    }

    fn display(&self) {
        println!("Payee: {} - Amount: {}", &self.payee, &self.amount);
    }
}

fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return input.trim().to_owned(),
        Err(e) => {
            println!("Unable to read input, {:?}", e);
            return String::new();
        }
    }
}

fn display_menu() {
    let menu = "
    \t My Billing App
    \t ==============
    \t 1. Add new Bill
    \t 2. Display Current Bills
    \n
    \t x. Exit
    \n
    \t Please select an option: ";

    print!("{}", menu);
    io::stdout().flush();
}

fn add_new_bill(bill_list: &mut Vec<Bill>) {
    println!("Please enter Payee Name: ");
    let payee = get_input();
    let mut amount: f32 = 0.0;
    loop {
        println!("Please enter amount owed: ");
        match get_input().parse::<f32>() {
            Ok(input) => {
                amount = input;
                break;
            }
            Err(_) => println!("Please enter a valid decimal number"),
        }
    }
    bill_list.push(Bill::new(&payee, amount));
}

fn display_bills(bill_list: &mut Vec<Bill>) {
    for bill in bill_list {
        bill.display();
    }
    println!("\n Press any key to continue");
    let _ = get_input();
}

enum MainMenu {
    AddBill,
    DisplayBill,

}

fn main() {
    let mut bills: Vec<Bill> = vec![];

    loop {
        display_menu();
        let menu_choice = get_input();
        match menu_choice.trim().to_lowercase().as_str() {
            "1" => add_new_bill(&mut bills),
            "2" => display_bills(&mut bills),
            "x" => break,
            _ => println!("\n Invalid Choice - Please try again"),
        }
    }
}
