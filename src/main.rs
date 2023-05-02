#![allow(non_camel_case_types)]
use project::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);
    if let Err(e) = run(&config) {
        eprintln!("文件错误: {e}");
        process::exit(1);
    };
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
