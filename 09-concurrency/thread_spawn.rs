use std::thread;

fn main() {
    thread::spawn(move || {
        println!("Hello from the goblin in the spawned thread!");
    });
    thread::sleep_ms(50);
}