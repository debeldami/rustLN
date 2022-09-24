// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

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
    let msg_hello = thread::spawn(move || msg_hello());
    let msg_thread = thread::spawn(move || msg_thread());
    let msg_excited = thread::spawn(move || msg_excited());

    match msg_hello.join() {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{:?}", err),
    }
    match msg_thread.join() {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{:?}", err),
    }
    match msg_excited.join() {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{:?}", err),
    }
}
