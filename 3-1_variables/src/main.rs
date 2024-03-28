fn main() {
    // Creating mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    // Changing the value of mutable variable
    x = 6;
    println!("The value of x is: {x}");

    // Creating const string variable
    const _CONSTANT_STRING: &str = "CONSTANT VARIABLE";

    // Creating const i32 variable
    const _CONSTANT_NUMBER: i32 = 123134514;

    // Shadowing
    let x = 5;
    println!("The value of x after shadowing is {x}"); 

    {
        let x = x * 3;
        println!("The value of x in inner scope is {x}");
    }

    println!("The value of x after completing execution of inner scope is {x}");
}
//asdasdasdasda
