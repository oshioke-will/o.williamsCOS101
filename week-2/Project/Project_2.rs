fn main() {
	let a:f64 = 450000.00;
	let b:f64 = 1500000.00;
	let c:f64 = 750000.00;
	let d:f64 = 2850000.00;
	let e:f64 = 250000.00;

	//Sum of the record
	let sum:f64 = a+b+c+d+e;
	println!("The Sum is {}",sum );

	//Average of the record
	let average:f64 = (a+b+c+d+e)/ 5.0;
	println!("The average is {}",average );
	
}