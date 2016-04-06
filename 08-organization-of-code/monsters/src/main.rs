extern crate monsters;
use monsters::Monster;
fn main() {
    monsters::print_from_monsters();
    let zmb1 = monsters::Zombie{health: 75, damage: 15};
    println!("我听见{}", zmb1.noise());
    zmb1.attack();
    println!("{:?}", zmb1);
}