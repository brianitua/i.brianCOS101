use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nIs the employee experienced? true/false ");
    io::stdin().read_line(&mut input1).expect("Line couldn't be read.");
    let experienced: bool = input1.trim().parse().expect("Couldn't parse experience input.");

    println!("\nHow old is the employee? ");
    io::stdin().read_line(&mut input2).expect("Line couldn't be read.");
    let age: i32 = input2.trim().parse().expect("Couldn't parse age input.");

    println!("Employee experience: {}", experienced);
    println!("Employee age: {}", age);

    if experienced == true && age >= 40
        {
            println!("\nIncentive: N1,560,000");
        }
    else if experienced == true && age >= 30 && age < 40
        {
            println!("\nIncentive: N1,480,000");
        }
    else if experienced == true && age < 28
        {
            println!("\nIncentive: N1,300,000");
        }
    else if experienced == false
        {
            println!("\nIncentive: N100,000");
        }
        else 
        {
            println!("\nYou are not qualified")
        }
}
