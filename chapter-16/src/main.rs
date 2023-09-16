use std::thread;
use std::sync::mpsc;
use std::time::Duration; 

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..25 {
//             println!("Hey, this is number {} from the thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..15 {
//         println!("Hi, this is number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap(); // Wait for the spawned thread to finish
// }

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();
// }


// Messages 
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone(); 

    // Thread 1 with transmitter 1
    thread::spawn(move || {
        let val = String::from("hi1");
        println!("Sending {}", val);
        tx.send(val).unwrap(); 

    });

    // Thread 1 with transmitter 1
    thread::spawn(move || {
        let val = String::from("hi2");
        println!("Sending {}", val);
        tx2.send(val).unwrap(); 
    });

    // let received = rx.recv().unwrap();

    for received in rx { // waits for all messages until 
        println!("Received {}", received);
    }
}