mod referencing;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let vi32 = vec![1, 2, 3];

    referencing::vector_reference()
}

