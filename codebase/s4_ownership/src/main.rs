/*
fn main() {
    let mut string_literal = "hello"; // declare a mutable string literal
    let mut string_value = String::from("hello"); // this is a String value

    string_value.push_str(", world!"); // append ", world!" to the string
    // string_literal.push_str(", world!"); // this line won't compile because string literals are immutable

    string_literal = "hello world"; // assign a new value to string_literal, 
    // string_literal += "!" // because string literals are immutable.

    println!("string literals: {}", string_literal); // prints "world"
    println!("String value: {}", string_value); 
}
*/

/*
// A better way to use `string_literal` and `String`
fn main() {
    let string_literal = "hello"; // this is a string literal
    let mut string_value = String::from("hello"); // this is a String value

    string_value.push_str(", world!"); // append ", world!" to the string
    // string_literal.push_str(", world!"); // this line won't compile because string literals are immutable

    println!("String literal: {}", string_literal);
    println!("String value: {}", string_value);
}
*/

/*
#[derive(Copy, Clone)]
struct MyNumber {
    x: i32,
}

fn main() {
    let x = MyNumber { x: 5 };
    let y = x;

    println!("x is: {}", x.x);
    println!("y is: {}", y.x);
}
*/

/*
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
*/

/*
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
*/

/* 
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
*/

/*
#[derive(Copy, Clone)]
struct MyString {
    s: String,
}

fn main() {
    let s1 = MyString { s: String::from("hello") };
    let s2 = s1;

    println!("s1 = {}, s2 = {}", s1.s, s2.s);
}
*/

/* 
fn main() { 

    let s = String::from("hello");  // s comes into scope

    println!("b4 takes_ownership s = {}", s);
    
    takes_ownership(s);             // s's value moves into the function... and so is no longer valid here

    // println!("after takes_ownership s = {}", s);                                

    let x = 5;                      // x comes into scope

    println!("b4 makes_copy(x) x = {}", x);

    makes_copy(x);                  // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    println!("after makes_copy(x) x = {}", x);                                

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
 */

 /* 
 fn main() {     
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("s1 = {}", s1);
    println!("s2 = {}", s2); // error[E0382]: borrow of moved value: `s2`
    println!("s3 = {}", s3);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}
 */


fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

/* 
fn main() {
    let mut s = String::from("abc");
    println!("1: s = {}", s);
    s = do_stuff(s);
    println!("3: s = {}", s);
}

fn do_stuff(s: String) -> String{
    println!("2: s = {}", s);
    s
} 
*/

/* 
fn main() {
    let s = String::from("abc");
    println!("1: s = {}", s);
    do_stuff(s);
    // println!("3: s = {}", s);
}

fn do_stuff(s: String) -> String{
    println!("2: s = {}", s);
    s
}
 */
