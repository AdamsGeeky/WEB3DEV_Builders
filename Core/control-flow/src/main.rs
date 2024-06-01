mod conditional;
mod loops;

fn main() {
    println!("Control Flow");

    println!("*** Conditional Statements ***");
    conditional::run_conditionals();
    println!("\n*** Loops ***");
    loops::run_loops();
}
