struct AlwaysEqual;

impl PartialEq for AlwaysEqual {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

fn main() {
    let subject = AlwaysEqual;
    let other = AlwaysEqual;

    if subject == other {
        println!("subject is equal to other");
    } else {
        println!("subject is not equal to other");
    }
}
