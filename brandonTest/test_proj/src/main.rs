use std::io::{self, Write};

fn main() {
    // immutable variable
    let x = 10;
    println!("x is immutable and currently: {x}");

    // mutable variable
    let mut y = 10;
    println!("y is mutable and currently: {y}");

    y = 20;
    println!("y is now: {y}");

    //incrementation
    y += 1;
    println!("y is now, after increment 1: {y}");

    //mutliplication
    y *= x;
    println!("y is now, after multiplying x: {y}");

    //constants must be declared with type and capitalized
    const A: i32 = 1;
    println!("Constant 'A' assigned: {A}");

    //shadowing and variable reassignment
    //variables can be redecleared when using let.
    //mutable variable can be redaclared to immutable.
    let mut z = 5;
    println!("The value of z is: {z}");

    z += 1;
    println!("The value of z after increment 1 is: {z}");

    let z = z + 1;

    {
        //variable still exist in shadowing, however loses the changes happened while shadowing
        println!("The value of z in the inner scope without redeclaration is: {z}");
        let mut z = z * 2;
        println!("The value of z in the inner scope is: {z}");
        z += 1;
        println!("The value of mutated z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");

    //the following is not allowed after redeclaring variable to immutable.
    //z -= 1;
    //println!("The value of z is: {z}");

    //Interger Literals
    // There are signed and unsigned variables.
    // (Signed are for positive and negatives, unsigned are for positives)
    // When interger overflow happens, wrapping happens if built.
    // Modulo is applied to the overflowing value.

    // Tupple
    println!("SECTION FOR TUPPLE");
    let tup: (i32, f64, u16) = (-420, 6.9, 420);

    // mutable tupple
    let (mut x, mut y, mut z) = tup;
    // let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    x += 1;
    y += 1.0;
    z += 1;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // Select tupple at index
    let a: (i64, f64, u16) = (400, 6.9, 32);

    let four_hundred = a.0;
    let six_point_nine = a.1;
    let thirty_two = a.2;

    println!("{four_hundred}, {six_point_nine}, {thirty_two}");

    let b: [i32; 5] = [11, 22, 33, 44, 55];

    let c = [3; 5];

    // print array
    println!("{:?}", b);

    // expanded print
    println!("{:#?}", c);

    let mut string_input = String::new();

    //flushing helps with determining where to start reading line
    print!("Enter a value between [0,4]: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut string_input)
        .expect("Failed to read line");

    let input: usize = string_input
        .trim()
        .parse()
        .expect("Index entered is not a number");

    println!("Found Value: {:?}", b[input]);
}
