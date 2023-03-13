//2: example 2 - Catch-all Patterns and the _ Placeholder
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {
    println!("Adding fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing fancy hat!");
}

fn reroll() {
    println!("Rerolling dice...");
}


/* 
//1: example 1 - Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = Some(6);
    let none: Option<i32> = None;

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}
 */