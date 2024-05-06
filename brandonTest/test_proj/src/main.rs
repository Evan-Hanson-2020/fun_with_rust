use rand::prelude::*;
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

    // Calling function with random param
    //holy_function(rand::random::<i32>());

    // commenting out for convinence.
    //holy_function(rand::thread_rng().gen_range(0..100));
    holy_function_2();

    // getting returns from function, returned variable with type i32
    let fn_return = first_function_return();
    println!("The returned value from function call: {fn_return}");

    // if statements can be used right side of let statement
    let condition = true;
    // both variabled when attempting to do if else must return the same type
    // The following will throw errors
    //let chosen_one = if condition { 420 } else { "I AM A STRING" };
    let chosen_one = if condition { 420 } else { 69 };

    println!("Chosen one is: {chosen_one}");

    println!("\n\n\n\nSECTION ABOUT LOOP");
    looping_function();
    while_function();
}

fn holy_function(x: i32) {
    println!("HOLY FUNCTION");
    println!("The function is given a param x with value: {x}");

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

fn holy_function_2() {
    let x = {
        let y = 6;

        println!("The value of y: {y}");

        // this returns the value y+1 when there are no semicolon at the end
        y + 1
    };

    println!("The value of x: {x}");

    // experimenting with return
    let z = {
        // Note we are not able to just return a variable that is just declared.
        // ie: let zz = 10 (without semicolon)
        let zz = 10;
        zz
    };

    println!("The value of z: {z}");
}

fn first_function_return() -> i32 {
    64
}

fn looping_function() {
    println!("Loop function called");
    let mut counter = 0;
    let result = loop {
        if counter == 10 {
            println!("Counter reached 10. Breaking...");
            break counter;
        }

        counter += 1;
        println!("Increment counter: {counter}");
    };
    println!("Loop result: {result}");
}

fn while_function() {
    println!("While function called");
    let mut counter = 0;
    // cannot use break in a while loop
    while counter < 10 {
        counter += 1;
        println!("Increment counter: {counter}");
    }

    println!("Counter: {counter}");
}
