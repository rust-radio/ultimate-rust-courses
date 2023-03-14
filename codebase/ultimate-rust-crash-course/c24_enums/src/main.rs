//2. Result
    /*
    #[must_use]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */
use std::fs::File;

fn main() {
    let res = File::open("foolll");
/* 
    let f = res.unwrap();
    println!("f is: {:?}", f);

    let f = res.expect("error message");
    println!("f is: {:?}", f);
 */

/* 
    if res.is_ok() {
        let f = res.unwrap();
        println!("f is: {:?}", f);
    } else {
        let err = res.err().unwrap();
        println!("Error: {:?}", err);
    }
 */
 
    match File::open("foolll") {
        Ok(f) => println!("f is: {:?}", f),
        Err(e) => println!("Error: {:?}", e),
    }
}

//  1. Option    
    /*
    enum Option<T> {
        Some(T),
        None,
    }
    */
/*     
fn main() {
 
    let mut x  = None;
    // let mut x: Option<i32> = None;
    println!("x is: {:?}", x);
    println!("x.is_none() is: {:?}", x.is_none());
    println!("x.is_some() is: {:?}", x.is_some());

    x = Some(5);
    println!("x is: {:?}", x);
    println!("x.is_some() is: {:?}", x.is_some());
    println!("x.is_none() is: {:?}", x.is_none());

    /* 
    for i in x {
        println!("{}", i);
    } 
    */

    if let Some(i) = x {
        println!("{}", i);
    }

}
 */

