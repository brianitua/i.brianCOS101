use std::io;
use std::io::Write;

fn main() {
    let mut input1 = String::new();
    println!("\nHow many drinks are you inputting for Lager?");
    
    io::stdin().read_line(&mut input1).expect("Error reading input");
    let _num = input1.trim().parse().expect("Error parsing");

    let mut _array1 = Vec::new();

    for _x in 0.._num {
        
        let mut input1 = String::new();
        println!("\nEnter drink {}?", _x+1);
        io::stdin().read_line(&mut input1).expect("Error reading input");
        let _entry = input1.trim();

        println!("\n{} is drink {}", _entry, _x+1);

        _array1.push(_entry.to_string());

}

        println!("\nHow many drinks are you inputting for stout?");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Error reading input");
        let _num = input1.trim().parse().expect("Error parsing");
    
        let mut _array2 = Vec::new();
    
        for _x in 0.._num {
            
            let mut input1 = String::new();
            println!("\nEnter drink {}?", _x+1);
            io::stdin().read_line(&mut input1).expect("Error reading input");
            let _entry = input1.trim();
    
            println!("\n{} is drink {}", _entry, _x+1);
    
            _array2.push(_entry.to_string());
    
                          }

        println!("\nHow many drinks are you inputting for Non-Alcoholic?");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Error reading input");
        let _num = input1.trim().parse().expect("Error parsing");
    
        let mut _array3 = Vec::new();
    
        for _x in 0.._num {
            
            let mut input1 = String::new();
            println!("\nEnter drink {}?", _x+1);
            io::stdin().read_line(&mut input1).expect("Error reading input");
            let _entry = input1.trim();
    
            println!("\n{} is drink {}", _entry, _x+1);
    
            _array3.push(_entry.to_string());
    
                         }
                


        println!("\nLager: ");
        for _a in &_array1 {
            println!("\n- {}", _a);
        }

        println!("\nStout: ");
        for _a in &_array2 {
            println!("\n- {}", _a);
        }

        println!("\nNon-Alcoholic: ");
        for _a in &_array3 {
            println!("\n- {}", _a);
        }

        let data = format!(
            "Lager:\n{}\n\nStout:\n{}\n\nNon-Alcoholic:\n{}\n",
            _array1.join("\n"),
            _array2.join("\n"),
            _array3.join("\n")
        );
       
        let mut file = std::fs::File::create("data.txt").expect("create failed");
        file.write_all(data.as_bytes()).expect("write failed");     
        println!("\nData written to file.");
    
}

