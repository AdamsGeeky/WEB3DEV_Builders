mod alike;
// Scope is a block of code that can be accessed from within that block.
fn main() {
    let x = 5;

    {
        let y = 10;
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }

    println!("The value of x is: {}", x);

    // y is not in scope here
    // println!("The value of y is: {}", y);

    let y = 6;

    println!("The value of y is: {}", y);
}
