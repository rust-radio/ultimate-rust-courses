
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("Integer Point: ({}, {})", both_integer.x, both_integer.y);
    println!("Float Point: ({}, {})", both_float.x, both_float.y);    
    println!("Integer and Float Point: ({}, {})", integer_and_float.x, integer_and_float.y);
    
    //  let sum = integer_and_float.x + integer_and_float.y; // error[E0277]: cannot add a float to an integer
}

/* 
// 1. 
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}
 */