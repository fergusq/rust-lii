use std::io::Read;
use std::fs::File;
use std::env;
extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^--?(?P<f>\w+)$").unwrap();

    let mut filename = None;
    for (_, arg) in env::args().enumerate() {
        match re.captures(&arg) {
            Some(caps) => { if handle_flag(caps.get(1).unwrap().as_str()) { return } },
            None => { filename = Some(arg.clone()) }
        }
    }

    let mut file = File::open(filename.expect("filename not given")).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content);
}

fn handle_flag(flag: &str) -> bool {
    match flag {
        "help" => {
            println!("usage: lii <file>");
            true
        }
        _ => {
            panic!("unknown option: {}", flag);
        }
    }
}