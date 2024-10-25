fn main() {
    let p:f64 = 520000000.0;  // Principal (N520,000,000)
    let r:f64 = 10.0;           // Annual interest rate (10%)
    let t:f64 = 5.0;            // Time (5 years)
    let n:f64 = 1.0;            // Compounded annually (n = 1)

    // compound interest formula
    let a = p * (1.0 + (r / (n * 100.0))).powf(n * t);
    println!("Amount is N{}", a);
    let ci = a - p;
    println!("Compound Interest is N{}", ci);
}
