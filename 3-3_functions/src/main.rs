fn main() {
    println!("Hello, world!");

    another_function();
    parameterized_function(5);
    print_labeled_measurement(5, "meters");
    // Char does not get converted to string automatically
    // print_labeled_measurement(123, 'm');

    // Statement - Does not return any value
    let _x = 6;

    // Expression
    let y = {
        let x = 3;
        x + 1 // Expression
        // x + 1; // Statement
    };
    println!("The value of y is {y}");

    println!("Printing the return value of get_five() function: {}", get_five());

    println!("Printing plus one value with parameter 6: {}", plus_one(6));
}

// Snake case for function names; Declaration can be above or below as long as it is in the scope
fn another_function() {
    println!("Another function");
}

fn parameterized_function(x: i32) {
    println!("The value passed is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is {value} {unit_label}");
}

fn get_five() -> i32 {
    5 // return value
}

fn plus_one(x: i32) -> i32 {
    x + 1
}