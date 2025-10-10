fn main() {
	let p:f64 = 510000000.00;
    let r:f64 = 5.00;
    let n:f64 = 3.00;

    // Depreciation
    let a:f64 = p * ( 1.0 - (r / 100.0)).powf(n);
    println!("THE DEPRECIATION IS {}", a);
   
}