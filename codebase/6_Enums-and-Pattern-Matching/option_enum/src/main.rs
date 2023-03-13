fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("some_number is: {:?}, some_char is: {:?}, absent_number is: {:?}", some_number, some_char, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // error[E0277]: cannot add `Option<i8>` to `i8`

    let sum1 = x + y.unwrap_or(0);

    let sum2 = match y {
        Some(value) => x + value,
        None => x,
    };
    println!("The sum1 is: {sum1}; the sum2 is: {sum2};");
}
