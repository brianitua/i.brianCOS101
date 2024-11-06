use std::io;

fn main() {
    let poundo = 3200;
    let fried = 3000;
    let amala = 2500;
    let eba = 2000;
    let white = 2500;
    
    let mut input_food = String::new();
    println!("What would you like to eat?");
    println!("p for Yam, f for Fried rice, a for Amala, e for Eba, w for White rice");
    io::stdin().read_line(&mut input_food).expect("Failed to read input");
    let food_choice = input_food.trim();

    let mut input_quantity = String::new();
    println!("How much would you like?");
    io::stdin().read_line(&mut input_quantity).expect("Failed to read input");
    let quantity: i64 = input_quantity.trim().parse().expect("Please enter a valid number");
    
    let price_per_item = if food_choice == "p" { poundo }
    else if food_choice == "f" { fried }
    else if food_choice == "a" { amala }
    else if food_choice == "e" { eba }
    else if food_choice == "w" { white }
    else { println!("Invalid food choice!");
        return;
};
    let mut total_price = price_per_item * quantity;
    
    if total_price > 10_000 {
        total_price = (total_price as f64 * 0.95) as i64;
    }
    
    println!("Total Price: N{}", total_price);
}
