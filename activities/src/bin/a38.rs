// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    use std::thread;

    let msg_one = thread::spawn(msg_hello);
    let msg_two = thread::spawn(msg_thread);
    let msg_three = thread::spawn(msg_excited);

    println!(
        "{}{}{}",
        msg_one.join().expect("Failed to join msg one"),
        msg_two.join().expect("Failed to join msg two"),
        msg_three.join().expect("Failed to join msg three")
    )
}
