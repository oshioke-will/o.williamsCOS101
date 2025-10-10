fn main() {
	let p:f64 = 520000000.0;
    let r:f64 = 10.0;
    let n:f64 = 5.0;

    // Amount
    let a:= p * ( 1.0 + (r / 100.0)).powf(n);
    println!("THE AMOUNT IS {}", a);
    let ci: = a - p;
    println!("THE COMPOUND INTEREST IS {}", ci);
}