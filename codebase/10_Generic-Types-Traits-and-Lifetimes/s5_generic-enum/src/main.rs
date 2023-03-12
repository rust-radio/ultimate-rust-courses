// Here's an example of how you might use the Result<T, E> enum in Rust:
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("enum.txt");

    let contents = match f {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => contents,
                Err(e) => e.to_string(),
            }
        }
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            "file not found".to_string()
        }
        Err(e) => e.to_string(),
    };

    println!("Contents: {}", contents);
}

/* 
// an example of how you might use the Option<T> enum in Rust:
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(n) => println!("x is {}", n),
        None => println!("x is None"),
    }

    match y {
        Some(n) => println!("y is {}", n),
        None => println!("y is None"),
    }
}
 */