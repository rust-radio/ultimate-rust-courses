// 2. Iterating over the Values in a Vector
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
}


// 1.err can’t have mutable and immutable references in the same scope. - error[E0502]
/* 
fn main() {    
    let mut v = vec![1, 2, 3, 4, 5];
    
    // let first = v[0]; // it is okay to clone it. 

    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

    let first = &v[0]; // it is okay to clone it. // immutable borrow later used here
    v.push(6); // mutable borrow occurs here
    println!("The first element is: {first}"); // immutable borrow later used here
}
 */

// 1.  Creating, Updating and Reading Elements of Vectors
/* 
fn main () {
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3,4];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {third}.");

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}."),
        None => println!("There is no fourth element.")
    }

    // let does_not_exist = &v[100];
    // println!("The `does_not_exist` is: {does_not_exist}.");

    let another_does_not_exist = v.get(101);
    match another_does_not_exist {
        Some(another_does_not_exist) => println!("The another_does_not_exist element is {}.", another_does_not_exist),
        None => println!("There is no another_does_not_exist element.")
    }
}
 */