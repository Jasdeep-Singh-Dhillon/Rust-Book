fn main() {
    println!("Hello, world!");

    another_function();
    parameterized_function(5);
}

// Snake case for function names; Declaration can be above or below as long as it is in the scope
fn another_function() {
    println!("Another function");
}

fn parameterized_function(x: i32) {
    println!("The value passed is {x}");
}