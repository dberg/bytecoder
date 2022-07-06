use crate::ast::{AttributeInfo, ClassFile, CpInfo, MethodInfo};
use crate::parser::{get_class_access_flags, get_method_access_flags};

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
        get_class_access_flags(class_file.access_flags).join(", "),
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
    for method_info in class_file.methods.iter() {
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
    let access_flags = get_method_access_flags(method_info.access_flags).join(" ");
    let method_name = get_constant_utf8(method_info.name_index, cp_info);
    let descriptor = get_constant_utf8(method_info.descriptor_index, cp_info);
    let attributes = method_info_attributes(method_info);
    format!("{} {}\n    \
        descriptor: {}\n    \
        flags: ({:#06x}) {}\n\
        {}",
        access_flags,
        method_name,
        descriptor,
        method_info.access_flags,
        access_flags,
        attributes.join("\n")
    )
}

fn method_info_attributes(method_info: &MethodInfo) -> Vec<String> {
    let mut attributes: Vec<String> = Vec::with_capacity(method_info.attributes_count as usize);
    for attribute in method_info.attributes.iter() {
        attributes.push(method_info_attribute(attribute));
    }
    attributes
}

fn method_info_attribute(attribute_info: &AttributeInfo) -> String {
    match attribute_info {
        AttributeInfo::ConstantValue { .. } => todo!(),
        code @ AttributeInfo::Code { .. } => method_attribute_info_code(code),
        AttributeInfo::StackMapTable { .. } => todo!(),
        AttributeInfo::Exceptions { .. } => todo!(),
        AttributeInfo::InnerClasses { .. } => todo!(),
        AttributeInfo::EnclosingMethod { .. } => todo!(),
        AttributeInfo::Synthetic { .. } => todo!(),
        AttributeInfo::Signature { .. } => todo!(),
        AttributeInfo::SourceFile { .. } => todo!(),
        AttributeInfo::SourceDebugExtension { .. } => todo!(),
        AttributeInfo::LineNumberTable { .. } => todo!(),
        AttributeInfo::LocalVariableTable { .. } => todo!(),
        AttributeInfo::LocalVariableTypeTable { .. } => todo!(),
        AttributeInfo::Deprecated { .. } => todo!(),
        AttributeInfo::RuntimeVisibleAnnotations { .. } => todo!(),
        AttributeInfo::RuntimeInvisibleAnnotations { .. } => todo!(),
        AttributeInfo::RuntimeVisibleParameterAnnotations { .. } => todo!(),
        AttributeInfo::RuntimeInvisibleParameterAnnotations { .. } => todo!(),
        AttributeInfo::RuntimeVisibleTypeAnnotations { .. } => todo!(),
        AttributeInfo::RuntimeInvisibleTypeAnnotations { .. } => todo!(),
        AttributeInfo::AnnotationDefault { .. } => todo!(),
        AttributeInfo::BootstrapMethods { .. } => todo!(),
        AttributeInfo::MethodParameters { .. } => todo!(),
        AttributeInfo::Module { .. } => todo!(),
        AttributeInfo::ModulePackages { .. } => todo!(),
        AttributeInfo::ModuleMainClass { .. } => todo!(),
        AttributeInfo::NestHost { .. } => todo!(),
        AttributeInfo::NestMembers { .. } => todo!(),
        AttributeInfo::Record { .. } => todo!(),
        AttributeInfo::PermittedSubclasses { .. } => todo!()
    }
}

fn method_attribute_info_code(code: &AttributeInfo) -> String {
    if let AttributeInfo::Code {
        attribute_name_index,
        attribute_length,
        max_stack,
        max_locals,
        code_length,
        code,
        exception_table_length,
        exception_table,
        attributes_count,
        attributes
    } = code {
        format!(
            "    Code:\n      \
            stack={}, locals={}, args_size=TODO\n\
            ",
            max_stack,
            max_locals
        )
    } else {
        panic!("Expected AttributeInfo::Code")
    }
}