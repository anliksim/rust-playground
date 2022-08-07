use std::cmp::Ordering;

fn main() {
    println!("# stings");
    string_matching();

    println!("# numbers");
    number_ordering();
}

fn string_matching() {
    let s = String::from("test");

    let if_res = if s == "test" { "passed" } else { "failed" };
    println!("if {if_res}");

    // need to convert String to &str
    let match_res = match s.as_str() {
        "test" => "passed",
        _ => "failed",
    };
    println!("match {match_res}");
}

fn number_ordering() {
    let num1: u32 = 1;
    let num2: u32 = 2;

    match num1.cmp(&num2) {
        Ordering::Less => println!("{num1} is less than {num2}"),
        Ordering::Greater => println!("{num1} is greater than {num2}"),
        Ordering::Equal => println!("{num1} is equal {num2}"),
    }
}
