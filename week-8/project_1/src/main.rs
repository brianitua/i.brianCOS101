use std::io;
fn main() {

        let office_administrator: Vec<&str> = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO",];
    
        let academic: Vec<&str> = vec!["-","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean",];
    
        let lawyer: Vec<&str> = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner",];
    
        let teacher: Vec<&str> = vec!["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Deputy Principal","Principal",];


    fn _exp() {

        let _public_servant: Vec<&str> = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES",];

        let mut input1: String = String::new(); 
        println!("\nWhat is your position? NOTE: type it as is, this is case sensitive!!");
        io::stdin().read_line(&mut input1).expect("error reading");
        let _level: &str = input1.trim();
    
        let mut input2: String = String::new();
            println!("\nHow many years of experience do you have?");
            io::stdin().read_line(&mut input2).expect("error reading");
            let _experience:i32 = input2.trim().parse().expect("Problem parsing");

            if _experience >= 1 && _experience <= 2 {
                println!("\nStaff position: {} \nLevel: {}", _level, _public_servant[0]);
            }
            else if  _experience >= 3 && _experience <= 5 {
                println!("\nStaff position: {} \nLevel: {}", _level, _public_servant[1]);
            }
            else if  _experience >= 6 && _experience <= 8 {
                println!("\nStaff position: {} \nLevel: {}", _level, _public_servant[2]);
            }
            else if  _experience >= 9 && _experience <= 10 {
                println!("\nStaff position: {} \nLevel: {}", _level, _public_servant[3]);
            }
            else if  _experience >= 10 && _experience <= 13 {
                println!("\nStaff position: {} \nLevel: {}", _level, _public_servant[4]);
            }
            else if  _experience > 13 {
                println!("\nStaff position: {} \nLevel: {}", _level, _public_servant[5]);
            }
            else {
                  println!("\nSo sorry I don't understand");
            }
    }
    

    let mut input1:String = String::new();

    println!("\nAre you an: office administrator, academic, lawyer or a teacher? (Note: lowercase please) ");
    io::stdin().read_line(&mut input1).expect("Error reading input");
    let _occupation = input1.trim();

    if _occupation == "lawyer" {
        println!("\n{:?}", lawyer);
        _exp();

    }
    else if _occupation == "office administrator" {
        println!("\n{:?}", office_administrator);
        _exp();
        }
    else if _occupation == "academic" {
        println!("\n{:?}", academic);
        _exp();
        }
    else if _occupation == "teacher" {
        println!("\n{:?}", teacher);
        _exp();
        }
        else {
            println!("\nWe don't have your occupaption in our registry, so sorry.")
        }
    }
