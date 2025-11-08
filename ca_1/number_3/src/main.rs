use std::io;

fn main() {
	println!("Welcome to the store!
		\nCode     Name        			Price
		\nR     Rust for Beginners       15000
		\nA     AI Basics                12500
		\nD     Data Structures in Rust  20000
		\nN     Networking Essentials    18000");

	//Input code
	println!("Please enter the book code");
	let mut code = String::new();
	io::stdin().read_line(&mut code).expect("Failed to read input");

	let r:i32 = 15000;
	let	a:i32 = 12500;
	let d:i32 = 20000;
	let n:i32 = 18000; 

	//Input Numbers
	println!("How many books do you want to purchase???");
	let mut amount = String::new();
	io::stdin().read_line(&mut amount).expect("Failed to read input");
	let amount:i32 = amount.trim().parse().expect("Not a number of books");
	println!("Your purchase is being processed");
if amount >= 3
 	{
 	let r:i32 = 15000 * 3;
	let	a:i32 = 12500 * 3;
	let d:i32 = 20000 * 3;
	let n:i32 = 18000 * 3;
	println!(""); 
 	}
 	if code = n
 	{
 		println!("Your price is{}", rust);
 	}
 	else if  {
 		println!("Your price is{}", aI);
 	}
 	
}
