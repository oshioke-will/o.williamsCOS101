fn main() {
    // Using Vec::new()
    let v: Vec<i32> = Vec::new();
    println!("\nThe length of Vec::new is: {}", v.len());

    // Using macro
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];
    println!("\nThe length of vec macro is: {}", v.len());
}