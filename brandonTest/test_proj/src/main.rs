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

    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");
}
