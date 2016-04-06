fn main() {
    let n = Box::new(42);
    let q = &*n;
    let p = &*n;
    println!("{:?}", q);
    println!("{:?}", p);
    println!("{:?}", n);

    let n1 = Box::new(33);
    let mut m = n1;
    *m = 67;
    println!("{:?}", m);
}