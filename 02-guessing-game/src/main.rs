extern crate rand; //等同use rand;

use std::io; //导入io库
use rand::Rng;

fn main() {
    println!("猜数字!");

    println!("请输入你的猜想。");

    //new是关联函数
    let mut guess = String::new(); //可变绑定，String可增长，UTF-8

    //如果前面没有use std::io，此书就是std::io::stdin()
    //read_line是方法，跟关联函数调用方法不同，使用点来调用，并且
    //方法是绑定到具体事例上的
    //引用默认是不可变的，因此使用&mut guess，而非&guess
    io::stdin().read_line(&mut guess)
        .ok()
        .expect("读取失败");

    println!("你猜的数字是{}", guess);

}
