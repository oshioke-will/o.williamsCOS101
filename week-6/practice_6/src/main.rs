fn main() {
    let s1 = String::from("Rust");
    let s2 = String::from("Lang");
    let result = s1 + &s2;
    println!("Concatenated: {}", result);
}