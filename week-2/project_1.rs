fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;
	// compound interest
	let a = p * (1.00 + (r / 100.00)).powf(n);
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound interest is {}", ci);
}
