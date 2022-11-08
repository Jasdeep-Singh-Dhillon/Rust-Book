use std::io::Write;

fn main() {
    // Looping to continue the conversion
    loop {
        // Getting the option from the user
        let option: u32 = get_number("Convert from\n\t1. Celsius to Fahrenheit\n\t2. Fahrenheit to Celsius\n\t3. Exit\n");

        // If option 1 then convert from celsius to fahrenheit
        if option == 1 {
            let value = get_number("Enter Celsius value: ");
            let value: f64 = value as f64;
            println!("The fahrenheit value is: {}", (value * 9.0 / 5.0) + 32.0);
        } 
        // If option 2 then convert from fahrenheit to celsius
        else if option == 2 {
            let value = get_number("Enter Fahrenheit value: ");            
            let value: f64 = value as f64;
            println!("The Celsius value is: {}", (value - 32.0) * 5.0 / 9.0);
        } 
        // If user entered something else break the loop and shut the program
        else {
            break;
        }  
    }
}

fn get_number(message: &str) -> u32 {
    // Looping until the user enters a numerical value
    loop {
        print!("{message}");
        // Flushing the output to print the value before the input
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        // Reading input from terminal
        match std::io::stdin().read_line(&mut input) {
            Ok(input) => input,
            Err(_) => {
                println!("Could not read line");
                continue;
            }
        };
        
        // Returning number as u32 if parsed correctly otherwise restart the loop
        return match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a numerical value");
                continue;
            }
        }
    }
}
