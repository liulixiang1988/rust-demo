fn main(){
    let player1 = "lixiang";
    let player2 = "tom";
    let player3 = player1.to_string() + player2;
    let player4 = player1 + player2; //an implementation of `std::ops::Add` might be missing for `&str`
}