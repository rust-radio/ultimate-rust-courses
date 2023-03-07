fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {}clear
     missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
