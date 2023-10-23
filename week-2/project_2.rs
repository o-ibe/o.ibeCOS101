fn main() {
	// item names
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;
	// quantity of items
	let qt:f64 = 2.00;
	let qm:f64 = 1.00;
	let qh:f64 = 3.00;
	let qd:f64 = 3.00;
	let qa:f64 = 1.00;
	// total quantity of items
	let q:f64 = qt + qm + qh + qd + qa;
	//sum of sales record
	let s = (t * qt) + (m * qm) + (h * qh) + (d * qd) + (a * qa);
	println!("The sum of the sales record of P.M. Okeke and Sons Ltd is {}", s);
	//average of sales record
	let a:f64 = s / q;
	println!("The average of the sales record of P.M. Okeke and Sons Ltd is {}", a);
}