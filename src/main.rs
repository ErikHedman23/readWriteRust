use std::fs;
use std::io::prelude::*;

fn main() {
    let _content = fs::read_to_string("plain.txt").unwrap();
    print!("contents is {}", _content);

    for line in _content.lines() {
        println!("line is {}", line);
    }

    let _content = fs::read("plain.txt").unwrap();
    print!("contents is {:?}", _content);

    // Writing to a file:
    let mut speech = String::new();

    speech.push_str("Hello there,");
    speech.push_str("My name is Erik");
    speech.push_str("I am writing to you to tell you");
    speech.push_str("YOu are AwSomesauce!");

    let _ = fs::write("speech.txt", speech);
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("plain.txt")
        .unwrap();
    let _ = file.write(b"\nYo");
    let newcontent = fs::read_to_string("plain.txt").unwrap();
    println!("{}", newcontent);
}
