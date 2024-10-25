fn main() {
    // Sales record amounts
    let sales = [
        450000.0 * 2.0,      // Toshiba
        250000.0 * 1.0,     // Acer
        1500000.0 * 1.0,   // Mac
        750000.0 * 3.0,   // HP
        2850000.0 * 3.0,  // Dell
    ];

    // Calculate the total number of units sold (frequency)
    let frequency: f64 = 2.0 + 1.0 + 1.0 + 3.0 + 3.0;

    // Calculate the total sales amount
    let total: f64 = sales.iter().sum();
    println!("Total sales amount: N{}", total);

    // Calculate the average sales amount
    let average: f64 = total / frequency;
    println!("Average sales amount: N{}", average);
}
