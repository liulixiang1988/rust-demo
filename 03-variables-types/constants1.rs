use std::f32::consts;
static MAX_HEALTH: i32 = 100;
static GAME_NAME: &'static str = "Monster Attack";

fn main() {
    //const PI: f32 = 3.14;
    println!("{}", consts::PI);
    println!("你在玩的游戏是：{}.", GAME_NAME);
    println!("你的最初生命值是：{}", MAX_HEALTH);
    println!("游戏{0}，生命值{1},多少点：{1}", GAME_NAME, MAX_HEALTH);
    println!("你有多少点健康值：{points}", points=70);
}