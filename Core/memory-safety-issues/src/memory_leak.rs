pub fn prevent_memory_leak() {
    let _x = Box::new(10);
}
