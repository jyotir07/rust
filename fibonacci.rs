fn main() {
    println!("{}", fib(2));
}
fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }
    for _ in 1..num-2 {  //using _ because we dont need to define a variable (like i) because we are not using it in the loop anywhere
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}
