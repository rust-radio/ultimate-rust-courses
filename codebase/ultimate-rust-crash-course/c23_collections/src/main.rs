use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let x = v.pop();
    println!("{:?}", x);
    println!("{}", v[1]);
    println!("{:?}", v);

    let mut h: HashMap <u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);
    println!("{:?}", h);

    let have_five = h.remove(&5).unwrap();
    println!("{}", have_five);
    println!("{:?}", h);
}
