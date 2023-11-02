// Rust program that tells how fast a car is traveling

fn main() {
    let dm1:f32 = 80.00; // first distance in miles
    let dm2:f32 = 120.00; // second distance in miles

    let t1:f32 = 2.00;
    let t2:f32 = 4.00;

    // kilometre conversion 
    let m:f32 = 1.60934; // m for multiplier 1mile = 1.60934km

    let dkm1:f32;
    dkm1 = dm1 * m; // first distance in kilometres
    println!("dkm1 = {} km",dkm1);

    let dkm2:f32;
    dkm2 = dm2 * m; // second distance in kilometres
    println!("dkm2 = {} km",dkm2);

    let spd1:f32;
    spd1 = dkm1 / t1; // first speed
    println!("Speed if car goes 80 miles in 2 hours is {} kmph",spd1);

    let spd2:f32;
    spd2 = dkm2 / t2; // second speed
    println!("Speed if car goes 120 miles in 4 hours is {} kmph",spd2);
}
