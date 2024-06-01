fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    let mut iter = v.iter();
    println!("{}", iter.next().unwrap()); // Prints: 1
    println!("{}", iter.next().unwrap()); // Prints: 2
    println!("{}", iter.next().unwrap()); // Prints: 3

    for i in &v {
        println!("{}", i);
    }
}
