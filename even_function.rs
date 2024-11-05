fn main() {
    let ans = is_even(7);
    println!("{}", ans);
    println!("{}", is_even(52));
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
