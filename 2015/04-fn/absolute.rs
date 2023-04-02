/// 计算绝对值
/// #例子
/// ```
/// let a = abs(-20);
/// ```
pub fn abs(x: i32) -> i32{
    if x > 0 {
        x
    } else {
        -x
    }
}

fn main(){
    let a = -30;
    println!("{}", abs(a));
}
