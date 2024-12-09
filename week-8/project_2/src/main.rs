use std::io;

fn main() { 

    let count = 0;
    let mut max_experience:i32 = 0;
    let mut max_number = String::new();

    while count == 0 {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nWhat's your interview number?");
    io::stdin().read_line(&mut input1).expect("Error reading line");
    let _number:String= input1.trim().parse().expect("error reading");


    println!("\nHow many years of experience do you have?");
    io::stdin().read_line(&mut input2).expect("Error reading line");
    let mut _experience:i32 = input2.trim().parse().expect("Error parsing");

    println!("\nAre you the final developer?");
    io::stdin().read_line(&mut input3).expect("Error reading line");
    let mut _final:&str = input3.trim();
                
                if _experience > max_experience {
                    max_experience = _experience;
                    max_number = _number;
                }

        if _final == "yes" {
            
            println!("\nCandidate {} has the most years of experience with {} years!", max_number, max_experience);
            break;
}


}

}