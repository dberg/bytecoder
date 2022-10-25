use crate::access_flags::MethodAccessFlag;
use crate::ast::{ClassFile, CpInfo, FieldType, FieldTypeTerm, MethodInfo};

pub fn get_constant_class_name(class_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_class = &cp_info[class_index as usize];
    if let CpInfo::ConstantClass { tag: _tag, name_index } = constant_class {
        get_constant_utf8(name_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantClass for idx {}", class_index)
    }
}

pub fn get_constant_utf8(name_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_utf8 = &cp_info[name_index as usize];
    if let CpInfo::ConstantUtf8 { tag: _tag, length: _length, bytes: _bytes, bytes_str } = constant_utf8 {
        bytes_str.clone()
    } else {
        panic!("Expected ConstantUtf8 for idx {}", name_index)
    }
}

pub fn get_name(name_and_type_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_name_and_type = &cp_info[name_and_type_index as usize];
    if let CpInfo::ConstantNameAndType { tag: _tag, name_index, descriptor_index: _descriptor_index } = constant_name_and_type {
        get_constant_utf8(name_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantNameAndType at idx {}", name_and_type_index)
    }
}

pub fn get_name_quoted(str: String) -> String {
    if str == "<init>" { format!("\"{}\"", str) } else { str }
}

pub fn get_type(name_and_type_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_name_and_type = &cp_info[name_and_type_index as usize];
    if let CpInfo::ConstantNameAndType { tag: _tag, name_index: _name_index, descriptor_index } = constant_name_and_type {
        get_constant_utf8(descriptor_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantNameAndType at idx {}", name_and_type_index)
    }
}

pub fn parse_method_arguments(method_info: &MethodInfo, cp_pool: &Vec<CpInfo>) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let descriptor = get_constant_utf8(method_info.descriptor_index, cp_pool);
    let mut open_found = false;
    let mut close_found = false;
    let mut beg = 0;
    for (i, c) in descriptor.chars().enumerate() {
        if c == '(' { open_found = true; beg = i + 1; continue; };
        if c == ')' { close_found = true; break; };
        if c == ';' {
            let arg: String = descriptor[beg..=i].to_string();
            args.push(arg);
            beg = i + 1;
        }
    }

    if !open_found || !close_found {
        panic!("Failed to parse method arguments {}", descriptor);
    }

    args
}

pub fn parse_field_types(field_types: &Vec<String>) -> Vec<FieldType> {
    field_types
        .iter()
        .map(|f| parse_field_type(f.to_string()))
        .collect()
}

pub fn parse_field_type(field_type: String) -> FieldType {
    let first_char = field_type.chars().next().unwrap();
    match first_char {
        'B' => FieldType::BaseType { term: FieldTypeTerm::B },
        'C' => FieldType::BaseType { term: FieldTypeTerm::C },
        'D' => FieldType::BaseType { term: FieldTypeTerm::D },
        'F' => FieldType::BaseType { term: FieldTypeTerm::F },
        'I' => FieldType::BaseType { term: FieldTypeTerm::I },
        'J' => FieldType::BaseType { term: FieldTypeTerm::J },
        'S' => FieldType::BaseType { term: FieldTypeTerm::S },
        'Z' => FieldType::BaseType { term: FieldTypeTerm::Z },
        'L' => {
            let class_name: String = field_type[1..].to_string();
            FieldType::ObjectType { class_name }
        },
        '[' => {
            let field_type_minus_reference = field_type[1..].to_string();
            let remaining_field_type = parse_field_type(field_type_minus_reference);
            FieldType::ArrayType { field_type: Box::new(remaining_field_type) }
        },
        _ => panic!("Failed to parse field type {}", field_type)
    }
}

pub fn method_arguments_count(method_info: &MethodInfo, cp_pool: &Vec<CpInfo>) -> usize {
    let args_len = parse_method_arguments(method_info, cp_pool).len();
    let flags = MethodAccessFlag::parse_flags(method_info.access_flags);
    let static_flag = flags.contains(&MethodAccessFlag::AccStatic);
    if static_flag { args_len } else { args_len + 1 }
}

pub fn method_info_return_type(descriptor_index: u16, class_file: &ClassFile) -> String {
    // ex.: ([Ljava/lang/String;)V
    // ex.: ()V
    let descriptor = get_constant_utf8(descriptor_index, &class_file.cp_info);
    let pos = descriptor.find(')');
    match pos {
        None => panic!("Failed to extract return type in descriptor {}", descriptor),
        Some(idx) => {
            descriptor.chars().skip(idx + 1).collect()
        }
    }
}

/// Reference:
/// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.3.3
pub fn return_descriptor_to_java_code(return_type: String) -> String {
    if return_type == "V" {
        String::from("void")
    } else {
        todo!("TODO: return_type {} to java code", return_type)
    }
}