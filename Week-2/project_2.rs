fn main() {
    // Sales record amounts
    let sales = [
        450000.0,      // Acer
        250000.0,     // Toshiba
        1500000.0,   // Mac
        750000.0,   // HP
        2850000.0, // Dell
            
    ];

    // Calculate the summation
    let total:f64 = sales.iter().sum();
    println!("Total sales amount: N{}", total);

    // Calculate the average
    let average:f64 = total / sales.len() as f64;
    println!("Average sales amount: N{}", average);
}
