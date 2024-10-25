fn main() {
    let p:f64 = 210_000.0;
    let r:f64 = 5.0;
    let n:i32 = 3;
    
    // Calculate the depreciation
    let a = p * (1.0 - (r / 100.0)).powi(n); //integer not float
    
    // Output the final value
    println!("The value of the TV after {} years is N{}", n, a);
}
