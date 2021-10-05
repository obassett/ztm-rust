// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new (state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "hibernate" => Some(PowerState::Hibernate),
            "off" => Some(PowerState::Off),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "sleep" => Some(PowerState::Sleep),
            _ => { None }
        }
    }
    
}

fn print_state_message(state: PowerState) {
    let message = match state {
        PowerState::Hibernate => "hibernating now",
        PowerState::Off => "powering down",
        PowerState::Reboot => "rebooting system",
        PowerState::Shutdown => "shutting down now",
        PowerState::Sleep => "I am so tired, I am going to sleep"
    };
    println!("{}", message)

}



fn main() {
    let mut buffer = String::new();
    use std::io;

    let input_ok = io::stdin().read_line(&mut buffer);
    if input_ok.is_ok() {
        match PowerState::new(&buffer) {
            Some(state) => print_state_message(state),
            None => println!("Invalid State")
        }

    } else {
        println!("Failed to read stdin")
    }
}
