//This program converts Temperatures

use std::io;

fn main() {
	println!("Temperature converter");

let mut temperature1 = String::new();

//input temperature
println!("Enter the temperature in Celsius");
io::stdin().read_line(&mut temperature1).expect("Not a valid input,Failed to read input");
let a:f64 = temperature1.trim().parse().expect("Not a valid input");

//this converts to farrhenheit
let f:f64 = (9.0/ 100.0) * a + 32.0;
println!("The temperature in farrhenheit is: {}", f);

//this converts to Kelvin
let k:f64 = a + 273.15;
println!("The temperature in farrhenheit is: {}", k);

if a < -0.0 {
	print!("It is at its freezing point", );
}

else if a > 0.0 && a <= 30.0
{
	print!("It is at its normal point", );
}
 else if a > 30.0 
{
	print!("It is at a very hot temperature", );
}

}
