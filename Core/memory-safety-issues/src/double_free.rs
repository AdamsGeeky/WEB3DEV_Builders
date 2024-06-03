pub fn prevent_double_free() {
    let x = Box::new(10);
    println!("Value: {}", x);
}
