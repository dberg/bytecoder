use crate::ast::{ClassFile, CpInfo};
use crate::parser_helper::{get_constant_class_name, get_name, get_name_quoted, get_type};

/// Read a MethodRef from the constant pool and return a string representing
/// the qualified name of the method along with the arguments. Examples of
/// output:
///   java/lang/Object."<init>":()V
///   java/lang/System.out:Ljava/io/PrintStream;
pub fn get_constant_method_ref_description(cp_index: usize, class_file: &ClassFile) -> String {
    match &class_file.cp_info[cp_index] {
        CpInfo::ConstantMethodref { tag: _tag, class_index, name_and_type_index } => {
            let class_name = get_constant_class_name(class_index.clone(), &class_file.cp_info);
            let method_name = get_name_quoted(get_name(name_and_type_index.clone(), &class_file.cp_info));
            let method_type = get_type(name_and_type_index.clone(), &class_file.cp_info);
            format!("{}.{}:{}", class_name, method_name, method_type)
        },
        _ => panic!("Unexpected type at cp_index {}", cp_index)
    }
}

/// Get the description for the `ldc` opcode.
/// Ex.: String Hello, World
pub fn get_ldc_description(cp_index: usize, class_file: &ClassFile) -> String {
    match &class_file.cp_info[cp_index] {
        CpInfo::ConstantString { tag: _tag, string_index} => {
            let str = get_ldc_description(string_index.clone() as usize, class_file);
            format!("String {}", str)
        },
        CpInfo::ConstantUtf8 { tag: _tag, length: _length, bytes: _bytes, bytes_str} => {
            bytes_str.clone()
        },
        _ => panic!("TODO: get_ldc_description cp_index {}", cp_index)
    }
}