use std::io::Write;
fn main() {
   
    let s1 = get_string();

    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);
}

fn get_string() -> String {
    print!("Please enter a string: ");
    let _ = std::io::stdout().flush();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    String::from(input.trim())
}

fn calculate_length(string: &String) -> usize {
    string.len()
}