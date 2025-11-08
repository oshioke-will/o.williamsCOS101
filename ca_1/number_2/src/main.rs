use std::io;

fn main() {
    println!("Employee salary calculator!");

    println!("Please input the name of the employee:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter the employee's number of hours worked:");
    let mut hours = String::new();
    io::stdin().read_line(&mut hours).expect("Failed to read input");
    let hours: i32 = hours.trim().parse().expect("Invalid number");

    let mut pay: i32;
    if hours <= 40 {
        pay = 3000 * hours;
    } else {
        pay = 4500 * hours;
    }

    if pay > 100000 {
        pay -= 2000; // Deduct tax
    }

    println!("{}'s net pay is: {}", name.trim(), pay);
}
