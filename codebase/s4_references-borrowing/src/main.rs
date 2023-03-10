fn main() {    
    let mut s = String::from("hello");

    println!("1: s = {}", s);

    change(&mut s);

    println!("2: s = {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/* 
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
 */
