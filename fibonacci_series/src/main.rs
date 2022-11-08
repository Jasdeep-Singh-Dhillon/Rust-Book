use std::io::Write;

fn main() {
    // Getting the number of iterations 
    let iterations = get_iterations();
    
    // Initializing vector with iterations value
    let mut f = vec![0; iterations];
    f[0] = 0;
    f[1] = 1;
    for i in 0..iterations {
        if i >= 2 {
            f[i] = f[i-1] + f[i-2];
        }
        print!("{} ", f[i]);
    }

}

// Function to get the number of iterations from user
fn get_iterations() -> usize {
    loop {
        let mut input = String::new();

        // Printing statement to the console to prompt the user
        print!("Please enter the number of iterations: ");
        // Flushing the output to console 
        let _ = std::io::stdout().flush();

        // Reading line entered by the user
        match std::io::stdin()
            .read_line(&mut input) {
                Ok(input) => input,
                Err(_) => {
                    println!("Could not read line");
                    continue;
            }
        };
        
        // Parsing the input and returning if parsed without error else restart the loop
        return match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a numerical value");
                continue;
            }
        };
    }
}