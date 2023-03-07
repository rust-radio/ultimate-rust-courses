fn main() {
    do_stuff(5.0, 12.0);
}

fn do_stuff(qty: f64, oz: f64) -> f64 {    
    println!("{}  {}-oz sarsaparillas(s)!", qty, oz);
    return qty * oz;
}