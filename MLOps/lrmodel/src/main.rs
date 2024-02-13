fn main() {
    println!("Welcome to Simple linear regression!");
    println!("This program will predict the value of y based on the given x value");
    println!("Enter x value");
    let mut x = String::new();
    std::io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    let x: f64 = x.trim().parse().expect("Please type a number!");
    println!("Enter y value");
    let mut y = String::new();
    std::io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line");
    let y: f64 = y.trim().parse().expect("Please type a number!");
    println!("The predicted value of y is {}", x * y);
    println!("Thank you for using Simple linear regression!");
}
