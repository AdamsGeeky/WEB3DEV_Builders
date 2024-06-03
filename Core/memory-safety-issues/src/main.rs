mod double_free;
mod memory_leak;
mod null_pointer;
mod race_condition;

fn main() {
    print!("** Memory Safety **\n");

    println!("Preventing Race Condition:");
    race_condition::prevent_race_condition();

    println!("\nPreventing Double Free:");
    double_free::prevent_double_free();

    println!("\nPreventing Memory Leak:");
    memory_leak::prevent_memory_leak();

    println!("\nPreventing Dangling Pointer:");
    null_pointer::prevent_dangling_pointer();

    println!("\nPreventing Null Pointer:");
    null_pointer::prevent_null_pointer();
}
