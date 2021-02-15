pub fn main() {
    let mut s = String::new();

    let s2 = String::from("initial contents");

    let data = "initial contents";

    let s3 = data.to_string();

    // the method also works on a literal directly:
    let s4 = "💩🚀".to_string();

    s.push_str(&s4);
    s.push_str(data);

    let s7 = String::from("Hello, ");
    let s8 = String::from("world!");
    let s9 = s7 + &s8; //

    println!("Data value: {}", data);

    println!("String value: {}", s);
    println!("String value: {}", s2);
    println!("String value: {}", s3);
    println!("String value: {}", s4);
    println!("String value: {}", s8);
    println!("String value: {}", s9);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Much better
    let s10 = format!("{}-{}-{}", s1, s2, s3);
    println!("String value: {}", s10);

    // String indexing doesnt work, so this following can panic if I do somethint wrong
    let hello = "Здравствуйте";
    let sh = &hello[0..4];
    // This panics: let sh = &hello[0..3];
    println!("String value: {}", sh);


    for c in hello.chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

}