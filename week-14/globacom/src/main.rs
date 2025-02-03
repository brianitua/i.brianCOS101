use std::io::{self, Write, Read};

fn main() {
    // Ask the user for their role
    print!("Enter your role: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

    let mut user_role = String::new();
    io::stdin().read_line(&mut user_role).unwrap();
    let user_role = user_role.trim(); // Remove the newline character

    // Based on the role, read the corresponding table
    if user_role == "administrator" {
        read_table("database_structure.sql");
    } else if user_role == "project_manager" {
        read_table("project_tb.sql");
    } else if user_role == "employee" {
        read_table("staff_tb.sql");  // File name for employee role
    } else if user_role == "customer" {
        read_table("customer_tb.sql");
    } else if user_role == "vendor" {
        read_table("data_plan_tb.sql");
    } else {
        println!("Invalid role");
    }
}

fn read_table(file_name: &str) {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
