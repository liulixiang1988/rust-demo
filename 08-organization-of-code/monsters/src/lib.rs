pub trait Monster {
    fn new(hlt: u32, dam: u32) -> Self;
    fn attack(&self);
    fn noise(&self) -> &'static str;
    fn attack_with_sound(&self){
        println!("声音：{}", self.noise());
    }
}

#[derive(Debug)]
struct Alien {
    health: u32,
    damage: u32
}

impl Monster for Alien {
    fn new(mut h: u32, d: u32) -> Alien{
        if h > 100 { h = 100; }
        Alien{health: h, damage: d}
    }
    fn attack(&self){
        println!("攻击{}点", self.damage);
    }

    fn noise(&self) -> &'static str{
        "Aaargh!"
    }
}

#[derive(Debug)]
pub struct Zombie {
    pub health: u32,
    pub damage: u32
}

impl Monster for Zombie {
    fn new(mut h: u32, d: u32) -> Zombie{
        if h > 100 { h = 100; }
        Zombie{health: h, damage: d}
    }

    fn attack(&self){
        println!("攻击{}点", 2*self.damage);
    }

    fn noise(&self) -> &'static str{
        "Aaargh!"
    }
}

pub fn print_from_monsters(){
    println!("Crate Monster Print!");
}