fn take_ownership(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    take_ownership(s);
    
    println!("{}", s); // ERROR: s was moved into the function
}