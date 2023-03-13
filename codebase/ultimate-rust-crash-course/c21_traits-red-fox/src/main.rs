use std::fmt;

trait Noisy {
    fn get_noise(&self) -> &str;
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str { "BYTE!" }
}

struct RedFox {
    enemy: bool,    
    life: u8,
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str { "Meow?" }
}

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

impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }
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

    print_noise(fox1); // prints "Meow?"
    print_noise(fox2); // prints "Meow?"
    print_noise(5_u8); // prints "BYTE!"    
    //print_noise(2_u16); // error[E0277]: the trait bound `u16: Noisy` is not satisfied
}
