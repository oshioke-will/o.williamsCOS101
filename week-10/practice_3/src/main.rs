fn create_string() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    let s1 = create_string();
    let s2 = s1; // s1 moved to s2
    
    println!("{}", s1); // ERROR: s1 no longer valid
}