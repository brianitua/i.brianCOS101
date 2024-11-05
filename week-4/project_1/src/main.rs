use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter decimal value for quadratic equation for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter decimal value for quadratic equation for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter decimal value for quadratic equation for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        // Two real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Roots: x1 = {} and x2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        // One real root
        let root = -b / (2.0 * a);
        println!("Root: x = {}", root);
    } else {
        // No real roots
        println!("There are no real roots.");
    }
}