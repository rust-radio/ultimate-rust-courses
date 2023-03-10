
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
