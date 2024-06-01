use colored::*;

pub fn run_loops() {
    // Loop example
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", format!("🔄 Loop result is: {}", result).blue());

    // While loop example
    let mut number = 3;
    while number != 0 {
        println!("{}", format!("{}️⃣!", number).green());
        number -= 1;
    }
    println!("{}", "🚀 LIFTOFF!!!".red().bold());

    // For loop example
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("{}", format!("🔢 The value is: {}", element).yellow());
    }

    for number in (1..4).rev() {
        println!("{}", format!("{}️⃣!", number).green());
    }
    println!("{}", "🚀 LIFTOFF!!!".red().bold());
}
