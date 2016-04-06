mod game1 {
    fn func1(){
        println!("我是不是可见的?");
    }

    pub fn func2(){
        println!("func2调用");
    }

    #[derive(Debug)]
    pub struct Magician {
        pub name: String,
        pub age: i32,
        power:i32
    }

    impl Magician {
        pub fn new(n: String, a: i32, p: i32) -> Magician {
            Magician { name: n, age: a, power: p}
        }
    }
}

fn main() {
    //game1::func1(); //错误
    game1::func2();
    let mag1 = game1::Magician::new("Liu Lixiang".to_string(), 28, 100);
    println!("{:?}", mag1);
}