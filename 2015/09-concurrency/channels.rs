use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let result = some_expensive_computation();
        tx.send(result).ok().expect("无法发送消息");
    });
    some_other_expensive_computation();
    if let Some(result) = rx.recv().ok(){
        println!("{:?}", result);
    }
}

fn some_expensive_computation() -> i32 {
    1
}

fn some_other_expensive_computation() {}