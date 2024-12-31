use std::io;
use std::io::Write;

fn main() {
    
    let mut _gen_array: Vec<String> = Vec::new();

    loop {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut temp_array = Vec::new();

    println!("\nWhat's your name?");
    io::stdin().read_line(&mut input1).expect("Error reading");
    let _name = input1.trim();
    temp_array.push(_name.to_string());

    println!("\nWhat's your matric number?");
    io::stdin().read_line(&mut input2).expect("Error reading");
    let _matric = input2.trim();
    temp_array.push(_matric.to_string());

    println!("\nWhat department are you in?");
    io::stdin().read_line(&mut input3).expect("Error reading");
    let _department = input3.trim();
    temp_array.push(_department.to_string());

    println!("\nWhat level are you in?");
    io::stdin().read_line(&mut input4).expect("Error reading");
    let _level: u32 = input4.trim().parse().expect("Error parsing");
    temp_array.push(_level.to_string());

    
    let deets = temp_array.join(", ");
    _gen_array.push(deets.to_string());
    let meets = _gen_array.join("\n");
    
    
    println!("\nAre we done here (yes/no)?");
    io::stdin().read_line(&mut input5).expect("Error reading");
    let _confirm = input5.trim();
         
         if _confirm == "yes" {
            println!("\nIn this order, the student's names, matric numbers, departments, and levels are:");
            println!("\n{}", meets);
            
        let mut file = std::fs::File::create("data.txt").expect("create failed");
        file.write_all(meets.as_bytes()).expect("write failed");     
        println!("\nData written to file.");
            break;
         }

    };
        
    
}

