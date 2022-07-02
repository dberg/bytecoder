use crate::ast::{ClassFile, CpInfo};

pub fn pretty_print_text(class_file: &ClassFile) {
    println!("TODO: public class A");
    println!("  minor version:{:x}\n  major version:{}\n  flags: {:x}\n  this_class: #{}\n  super_class: #{}\n  interfaces: {}, fields: {}, methods: {}, attributes: {}",
        class_file.minor_version,
        class_file.major_version,
        class_file.access_flags,
        class_file.this_class,
        class_file.super_class,
        class_file.interfaces_count,
        class_file.fields_count,
        class_file.methods_count,
        class_file.attributes_count,
    );

    println!("Constant pool({}):", class_file.constant_pool_count - 1);

    for (idx, item) in class_file.cp_info.iter().enumerate() {
        if idx != 0 {
            println!("  #{}:{:?}", idx, item);
        }
    }
}

fn cp_info_to_string(idx: usize, cp_info: &Vec<CpInfo>) -> String {
    match cp_info[idx] {
        CpInfo::ConstantClass { .. } => todo!(),
        CpInfo::ConstantFieldref { .. } => todo!(),
        CpInfo::ConstantMethodref { tag: _tag, class_index, name_and_type_index } =>
            format!("Methodref\t#{}.#{}\t// {}.{}", class_index, name_and_type_index, get_constant_class_name(class_index, cp_info), "TODO_METHOD_NAME"),
        CpInfo::ConstantString { .. } => todo!(),
        CpInfo::ConstantNameAndType { .. } => todo!(),
        CpInfo::ConstantUtf8 { .. } => todo!(),
    }
}

fn get_constant_class_name(class_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_class = &cp_info[class_index as usize];
    if let CpInfo::ConstantClass { tag, name_index } = constant_class {
        get_constant_utf8(name_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantClass for idx {}", class_index)
    }
}

fn get_constant_utf8(name_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_utf8 = &cp_info[name_index as usize];
    if let CpInfo::ConstantUtf8 { tag, length, bytes, bytes_str } = constant_utf8 {
        bytes_str.clone()
    } else {
        panic!("Expected ConstantUtf8 for idx {}", name_index)
    }
}