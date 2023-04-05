use std::convert::TryInto;

fn main() {
    println!("Hello, world!");
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("{} + {} + {} + {} = {}", a, b, c, d, e);

    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12c;
    println!("base 10 {} {} {}", three, thirty, three_hundred);
    println!("base 2 {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8 {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16 {:x} {:x} {:x}", three, thirty, three_hundred);

    let aa: i32 = 10;
    let bb: u16 = 20;

    if aa > bb as i32 {
        println!("aa > bb");
    } else {
        println!("aa <= bb");
    }

    let bb_ = bb.try_into().unwrap();
    if aa < bb_ {
        println!("aa < bb");
    } else {
        println!("aa >= bb");
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
