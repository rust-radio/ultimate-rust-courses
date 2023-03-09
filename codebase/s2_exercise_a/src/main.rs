
fn do_stuff(qty: f64, oz: f64) -> f64 {    
    println!("{}  {}-oz sarsaparillas(s)!", qty, oz);
    qty * oz
}

fn main() {
    let x = do_stuff(5.0, 12.0);

    println!("The value of x is: {x}");
}


/*  
fn do_stuff(qty: f64, oz: f64) -> Result<f64, String> {
    println!("{}  {}-oz sarsaparillas(s)!", qty, oz);
    if oz == 0.0 {
        return Err("Bad things happen".to_string());
    }

    Ok(qty * oz)
}

fn main() {
    let result = do_stuff(5.0, 12.0);
    match result {
        Ok(value) => println!("The result is {}", value),
        Err(message) => println!("Error: {}", message),
    }
}

 */