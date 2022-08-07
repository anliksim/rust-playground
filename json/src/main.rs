use std::fs::{self, File};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    println!("# data string");
    from_data_string();

    println!("# file");
    from_file();

    println!("# generate");
    generate_file();
}

fn generate_file() {
    let res: Vec<Person> = (1..5)
        .map(|x| Person {
            age: x,
            name: "Name".to_owned(),
            phones: ["+44 1234567".to_owned()].to_vec(),
        })
        .collect();

    let file = File::create("ignore_output.json").expect("Error creating file");
    serde_json::to_writer(&file, &res).expect("Error writing to file");
}

fn from_file() {
    let contents = fs::read_to_string("input.json").expect("Error reading file");

    let typed_result: Person = serde_json::from_str(&contents).unwrap();
    println!("{:?}", typed_result);
}

fn from_data_string() {
    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }
    "#;

    let untyped_result: Value = serde_json::from_str(data).unwrap();
    println!("{}", untyped_result);

    let typed_result: Person = serde_json::from_str(data).unwrap();
    println!("{:?}", typed_result);
}
