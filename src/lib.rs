use crate::parser::ast::ClassFile;

mod parser;

pub fn parse_class_file(bytecode: &Vec<u8>) -> ClassFile {
    parser::parse_class_file(bytecode)
}

