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

    // TMP for debugging
    for (idx, item) in class_file.cp_info.iter().enumerate() {
        if idx != 0 {
            println!("  #{}:{:?}", idx, item);
        }
    }

    println!("\n\nWIP: Constant pool items");
    for (idx, _item) in class_file.cp_info.iter().enumerate() {
        if idx != 0 {
            let line = cp_info_to_string(idx, &class_file.cp_info);
            println!("{}", line);
        }
    }
}

fn cp_info_to_string(idx: usize, cp_info: &Vec<CpInfo>) -> String {
    match cp_info[idx] {
        CpInfo::ConstantClass { .. } => todo!(),
        CpInfo::ConstantFieldref { .. } => todo!(),
        CpInfo::ConstantMethodref { tag: _tag, class_index, name_and_type_index } => {
            let class_name = get_constant_class_name(class_index, cp_info);
            let method_name = get_method_name(name_and_type_index, cp_info);
            let method_type = get_method_type(name_and_type_index, cp_info);
            format!("  #{} = Methodref\t#{}.#{}\t// {}.{}:{}", idx, class_index, name_and_type_index, class_name, method_name, method_type)
        },
        CpInfo::ConstantString { .. } => todo!(),
        CpInfo::ConstantNameAndType { tag: _tag, name_index, descriptor_index } =>
            format!("  #{} NameAndType\t#{}:#{}\t//TODO", idx, name_index, descriptor_index),
        CpInfo::ConstantUtf8 { .. } => todo!(),
    }
}

fn get_constant_class_name(class_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_class = &cp_info[class_index as usize];
    if let CpInfo::ConstantClass { tag: _tag, name_index } = constant_class {
        get_constant_utf8(name_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantClass for idx {}", class_index)
    }
}

fn get_constant_utf8(name_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_utf8 = &cp_info[name_index as usize];
    if let CpInfo::ConstantUtf8 { tag: _tag, length: _length, bytes: _bytes, bytes_str } = constant_utf8 {
        bytes_str.clone()
    } else {
        panic!("Expected ConstantUtf8 for idx {}", name_index)
    }
}

fn get_method_name(name_and_type_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_name_and_type = &cp_info[name_and_type_index as usize];
    if let CpInfo::ConstantNameAndType { tag: _tag, name_index, descriptor_index: _descriptor_index } = constant_name_and_type {
        get_constant_utf8(name_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantNameAndType at idx {}", name_and_type_index)
    }
}

fn get_method_type(name_and_type_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_name_and_type = &cp_info[name_and_type_index as usize];
    if let CpInfo::ConstantNameAndType { tag: _tag, name_index: _name_index, descriptor_index } = constant_name_and_type {
        get_constant_utf8(descriptor_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantNameAndType at idx {}", name_and_type_index)
    }
}