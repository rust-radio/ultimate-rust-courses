use std::fmt;

// a `RedFox` struct
struct RedFox {
    enemy: bool,    
    life: u8,
}

//  implements the `fmt::Display` trait for a `RedFox` struct.
impl fmt::Display for RedFox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RedFox {{ 
                enemy: {}, 
                life: {} 
            }}",
            self.enemy, self.life )
    }
}

// constructor 
impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }
    /*
    // associated function
    fn function();
    fn move(self);
    fn borrow(&self);
    fn mut_borrow(&mut self);
    */
}


fn main() {
    let fox1 = RedFox {
        enemy: false,
        life: 30,
    };
    println!("fox1: {}", fox1);
    
    let mut fox2 = RedFox::new();
    println!("fox2: {}", fox2);  
    
    let life_left = fox2.life;
    fox2.enemy = false;
    //fox.some_method();

    println!("life_left: {}", life_left); 
    println!("fox2: {}", fox2); 
}
