


pub fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    assert_eq!(some_string.is_some(), true);
    assert_eq!(some_number.is_some(), true);

    println!("option::some .unwrapped() = {}", some_number.unwrap());

    let absent_number: Option<i32> = None;

    assert_eq!(absent_number.is_some(), false);
    assert_eq!(absent_number.is_none(), true);


}
