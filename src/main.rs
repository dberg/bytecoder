#![allow(dead_code)]

use std::{env, fs};
use crate::parser::parse_class_file;
use crate::pretty_print::pretty_print_text;

mod parser;
mod pretty_print;
mod pretty_print_helper;

fn main() {
    let class_filename = env::args().nth(1).expect("Missing class filename argument");
    let class_file_contents = fs::read(class_filename).expect("Failed to read class filename");
    let class_file = parse_class_file(&class_file_contents);
    pretty_print_text(&class_file);
}
