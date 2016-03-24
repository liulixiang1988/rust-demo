struct Player {
    nname: &'static str, //nickname
    health: i32,
    level: u8
}
fn main() {
    //tuple struct
    struct Score(i32, u8);
    let score1 = Score(73, 2);
    let Score(h, l) = score1;
    println!("Health:{} Level:{}", h, l);

    let mut pl1 = Player{nname: "Liulx", health: 73, level: 2};
    println!("Player {} is at level {}", pl1.nname, pl1.level);

    //point dereferencing
    let ps = &Player{ nname: "John", health: 95, level: 1 };
    println!("{} == {}", ps.nname, (*ps).nname);
}