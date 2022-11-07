use std::io::Write;

fn main() {
    let number: u32;

    loop {
        let mut input = String::new();

        print!("Enter a number: ");
        let _ = std::io::stdout().flush();
        match std::io::stdin()
            .read_line(&mut input) {
                Ok(input) => input,
                Err(_) => {
                    println!("Failed to read line");
                    continue;
                }
            };
        match input.trim().parse() {
            Ok(num) => {
                number = num;
                break;
            }
            Err(_) => {
                println!("Please enter a numerical value");
                continue;
            }
        }             
    } 

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
