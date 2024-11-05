fn get_string_length(s: String) -> usize {
    s.chars().count()  //implicit return of a value (without using "return" or ';')
}

fn main() {
    let my_string = String::from("Heyy!");
    let length = get_string_length(my_string);
    println!("Length is: {}", length);
}
