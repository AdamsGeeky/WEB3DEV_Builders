pub fn prevent_dangling_pointer() {
    fn get_pointer() -> Option<&'static i32> {
        None
    }

    if let Some(ptr) = get_pointer() {
        println!("{}", ptr);
    } else {
        println!("No valid pointer");
    }
}

pub fn prevent_null_pointer() {
    let x: *const i32 = std::ptr::null();
    println!("{:p}", x);
}
