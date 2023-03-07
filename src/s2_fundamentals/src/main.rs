fn main() {    
    let mut bunnies = 16;
    println!("Hello, world! The mutable variable bunnies is: {bunnies}");
    bunnies = 2;
    const WARP_FACTOR: f64 = 9.9;
    println!("Hello, world! The mutable variable bunnies is: {bunnies}, The const immutable variable is: {WARP_FACTOR}");
    let mut x = 5; // x is mutable
    {
        let y = 99;
        println!("{}, {}", x, y);
        let x = 99;
        println!("{}", x); // print "99"
    }
    println!("{}", x); // print "5"
    // println!("{}, {}", x, y); // error[E0425]: cannot find value `y` in this scope
    let x = x; // x is now immutable

}