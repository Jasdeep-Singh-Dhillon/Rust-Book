fn main() {
    // Floating point
    let _x = 6.5; //f64 - default when assigning value with decimal points
    let _y: f32 = 3.4; //f32 - explicit type annotation

    // Boolean
    let _t = true;
    let _f: bool = false; // explicit type annotation

    // Character
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
                        // Char uses UTF-8 encoded text
    let _heart_eyed_cat = '😻';

    // Compound Types

    // Tuple
    let tup: (i32, f32, u8) = (500, 541.1, 4);
    let (_x, y, _z) = tup; // Destructuring

    println!("The value of y is {y}");
    println!("The first value of tup is {}", tup.0);

    // Arrays
    let _a = [1, 2, 3, 4, 5]; // Stack allocated
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Explicit data type and size
    println!("First value of array a: {}", a[0]);
    let _size = 10;
    let a = [3; 5]; // Array created with 5 elements and value 3
    println!("Array a: {:?}", a);
}
