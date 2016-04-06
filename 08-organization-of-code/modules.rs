mod game1 {
    fn func1(){
        println!("我是不是可见的?");
    }

    pub fn func2(){
        println!("func2调用");
    }
}

fn main() {
    //game1::func1(); //错误
    game1::func2();
}