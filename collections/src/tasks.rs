
pub fn main() {
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary
    let mut list = vec![1,5,6,2,9,10,15,19];
    let mut sum = 0;
    for i in &list {
        sum += i;
    }
    println!("Sum: {:?}", sum);
    println!("Average: {:?}", sum/list.len());

    list.sort();
    let mid = list.len() / 2;

    let median = list[mid];
    println!("Median: {}", median)
}