use std::collections::HashMap;
use crate::ast::{ClassFile, CpInfo, MethodInfo};

pub fn pretty_print_text(class_file: &ClassFile) {
    println!("TODO: public class A");
    println!("  \
      minor version: {:x}\n  \
      major version: {}\n  \
      flags: ({:#06x}) {}\n  \
      this_class: #{}\t// {}\n  \
      super_class: #{}\t// {}\n  \
      interfaces: {}  fields: {}, methods: {}, attributes: {}\
      ",
        class_file.minor_version,
        class_file.major_version,
        class_file.access_flags,
        get_flags_description(class_file.access_flags),
        class_file.this_class,
        get_constant_class_name(class_file.this_class, &class_file.cp_info),
        class_file.super_class,
        get_constant_class_name(class_file.super_class, &class_file.cp_info),
        class_file.interfaces_count,
        class_file.fields_count,
        class_file.methods_count,
        class_file.attributes_count,
    );

    println!("Constant pool({}):", class_file.constant_pool_count - 1);
    for (idx, _item) in class_file.cp_info.iter().enumerate() {
        if idx != 0 {
            let line = cp_info_to_string(idx, &class_file.cp_info);
            println!("{}", line);
        }
    }

    // TMP debugging
    for method_info in class_file.methods.iter() {
        println!("{:?}", method_info);
    }

    println!("{{");
    for (idx, method_info) in class_file.methods.iter().enumerate() {
        let method_str = method_info_to_string(method_info, &class_file.cp_info);
        println!("  {}", method_str);
    }
    println!("}}");
}

fn cp_info_to_string(idx: usize, cp_info: &Vec<CpInfo>) -> String {
    match &cp_info[idx] {
        CpInfo::ConstantClass { tag: _tag, name_index } =>
            format!("  #{} = Class\t#{}\t// {}", idx, name_index, get_constant_utf8(name_index.clone(), cp_info)),
        CpInfo::ConstantFieldref { tag, class_index, name_and_type_index } => {
            let class_name = get_constant_class_name(class_index.clone(), cp_info);
            let field_name = get_name(name_and_type_index.clone(), cp_info);
            let field_type = get_type(name_and_type_index.clone(), cp_info);
            format!("  #{} = Fieldref\t#{}.#{}\t// {}.{}:{}", idx, class_index, name_and_type_index, class_name, field_name, field_type)
        },
        CpInfo::ConstantMethodref { tag: _tag, class_index, name_and_type_index } => {
            let class_name = get_constant_class_name(class_index.clone(), cp_info);
            let method_name = get_name(name_and_type_index.clone(), cp_info);
            let method_type = get_type(name_and_type_index.clone(), cp_info);
            format!("  #{} = Methodref\t#{}.#{}\t// {}.{}:{}", idx, class_index, name_and_type_index, class_name, method_name, method_type)
        },
        CpInfo::ConstantString { tag, string_index } =>
            format!("  #{} String = #{}\t// {}", idx, string_index, get_constant_utf8(string_index.clone(), cp_info)),
        CpInfo::ConstantNameAndType { tag: _tag, name_index, descriptor_index } => {
            let name = get_constant_utf8(name_index.clone(), cp_info);
            let typename = get_constant_utf8(descriptor_index.clone(), cp_info);
            format!("  #{} = NameAndType\t#{}:#{}\t// {}:{}", idx, name_index, descriptor_index, name, typename)
        },
        CpInfo::ConstantUtf8 { tag: _tag, length: _length, bytes: _bytes, bytes_str } =>
            format!("  #{} = Utf8\t{}", idx, bytes_str)
    }
}

fn get_flags_description(flags: u16) -> String {
    let mut flags_desc = get_flags(flags);
    flags_desc.sort();
    flags_desc.join(", ")
}

fn get_flags(flags: u16) -> Vec<String> {
    let flag_to_description: HashMap<u16, &str> = HashMap::from([
        (0x0001, "ACC_PUBLIC"),
        (0x0010, "ACC_FINAL"),
        (0x0020, "ACC_SUPER"),
        (0x0200, "ACC_INTERFACE"),
        (0x0400, "ACC_ABSTRACT"),
        (0x1000, "ACC_SYNTHETIC"),
        (0x2000, "ACC_ANNOTATION"),
        (0x4000, "ACC_ENUM"),
        (0x8000, "ACC_MODULE"),
    ]);
    let mut flags_description: Vec<String> = Vec::new();
    for (f, v) in flag_to_description {
        if f & flags != 0 {
            flags_description.push(String::from(v));
        }
    }
    flags_description
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

fn get_name(name_and_type_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_name_and_type = &cp_info[name_and_type_index as usize];
    if let CpInfo::ConstantNameAndType { tag: _tag, name_index, descriptor_index: _descriptor_index } = constant_name_and_type {
        get_constant_utf8(name_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantNameAndType at idx {}", name_and_type_index)
    }
}

fn get_type(name_and_type_index: u16, cp_info: &Vec<CpInfo>) -> String {
    let constant_name_and_type = &cp_info[name_and_type_index as usize];
    if let CpInfo::ConstantNameAndType { tag: _tag, name_index: _name_index, descriptor_index } = constant_name_and_type {
        get_constant_utf8(descriptor_index.clone(), cp_info)
    } else {
        panic!("Expected ConstantNameAndType at idx {}", name_and_type_index)
    }
}

fn method_info_to_string(method_info: &MethodInfo, cp_info: &Vec<CpInfo>) -> String {
    let access_flags = get_flags(method_info.access_flags).join(", ");
    let method_name = get_constant_utf8(method_info.name_index, cp_info);
    let descriptor = get_constant_utf8(method_info.descriptor_index, cp_info);
    format!("{} {}\n    descriptor: {}\n    flags: ({:#06x}) {}\nTODO",
        access_flags,
        method_name,
        descriptor,
        method_info.access_flags,
        access_flags,
    )
}