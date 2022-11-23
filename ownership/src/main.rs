fn main() {
    let s = String::from("Hello");

    takes_ownership(s.clone());

    let x = 5;
    makes_copy(x);

    println!("String: {}", s);
    
    {
        let s1 = s.clone();
        println!("{}", s1);
    }
    println!("String: {}", s);
    println!("String: {}", s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
