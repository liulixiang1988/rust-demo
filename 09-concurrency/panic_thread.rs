use std::thread;

fn main() {
    let result = thread::spawn(move || {
        panic!("Panic!");
    }).join();
    if result.is_err() {
        println!("子线程发生了panic!");
    }
}