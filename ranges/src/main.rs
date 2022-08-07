use std::ops::Range;

fn main() {
    let excl: Vec<i32> = (1..10).collect();
    println!("Exclusive range 1..10: {:?}", excl);

    let incl: Vec<i32> = (1..=10).collect();
    println!("Inclusive range 1..=10: {:?}", incl);

    let mut range: Range<i32> = 1..4;
    let first: i32 = range.next().unwrap();
    let second: i32 = range.next().unwrap();
    println!("Range 1..10 first: {first}, second: {second}");
}
