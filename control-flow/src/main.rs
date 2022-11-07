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

    // If statement requires a bool value or boolean expression
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 2, 3 or 4");
    }
}
