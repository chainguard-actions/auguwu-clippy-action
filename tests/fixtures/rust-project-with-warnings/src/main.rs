fn main() {
    // This will trigger clippy::needless_return warning
    let x: Vec<i32> = Vec::new();
    println!("len: {}", x.len());
}
