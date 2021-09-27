use std::ffi::OsString;

// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(100.0, "Ricardo".to_owned()),
        Ticket::Standard(50.0),
        Ticket::Vip(80.0, "Estella".to_owned()),
    ];

    // * Print out a list of tickets and their information for an event

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage Ticket: Holder: {:?}, Price ${:?}", holder, price)
            }
            Ticket::Vip(price, holder) => {
                println!("VIP Ticket: Holder: {:?}, Price ${:?}", holder, price)
            }
            Ticket::Standard(price) => println!("Standard Ticket: Price ${:?}", price),
        }
    }
}
