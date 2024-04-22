#[test]
pub fn string_test() {
    let my_string = String::from("Hello, World!");

    for c in my_string.chars() {
        println!("{}", c);
    }
}

