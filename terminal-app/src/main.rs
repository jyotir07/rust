fn main() {
    // println!("Hello, world!");
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    println!("Pattern: {:?}, Path: {:?}", pattern, path)
}
