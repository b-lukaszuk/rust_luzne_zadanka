use std::sync::mpsc; // mpsc - multiple producer, single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val); // error cannot be shure that the receiver
                                    // didn't drop the val, e.g 3-4 lines below
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
