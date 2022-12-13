#![allow(dead_code)]

use crate::parser::{get_u1, get_u2};
use crate::parser::access_flags::{ClassAccessFlag, MethodAccessFlag};
use crate::parser::access_flags::ClassAccessFlag::AccSuper;
use crate::parser::ast::{AttributeInfo, ClassFile, CpInfo, MethodInfo};
use crate::parser::opcodes::{get_opcode, Opcode};
use crate::parser::parser_helper::{get_constant_class_name, get_constant_utf8, get_name, get_name_quoted, get_type, method_arguments_count, method_info_return_type, parse_field_types, parse_method_arguments, return_descriptor_to_java_code};
use crate::pretty_print_helper::{get_constant_method_ref_description, get_ldc_description, get_static_description};

pub fn pretty_print_text(class_file: &ClassFile) {
    let this_class_name = get_constant_class_name(class_file.this_class, &class_file.cp_info);
    let this_class = format!("this_class: #{}", class_file.this_class);
    let super_class = format!("super_class: #{}", class_file.super_class);

    let access_flags: Vec<ClassAccessFlag> = ClassAccessFlag::parse_flags(class_file.access_flags);
    let this_class_access_flags: Vec<ClassAccessFlag> = access_flags.iter().filter(|&class_access_flag| class_access_flag != &AccSuper).cloned().collect();
    let this_class_access_flags: String = this_class_access_flags.iter().map(|f| f.to_java_code()).collect();
    let class_access_flags: Vec<&str> = access_flags.iter().map(|f| f.to_str()).collect();
    let class_access_flags: String = class_access_flags.join(", ");

    println!("Classfile TODO");
    println!("{} class {}\n  \
      minor version: {:x}\n  \
      major version: {}\n  \
      flags: ({:#06x}) {}\n  \
      {:<40}// {}\n  \
      {:<40}// {}\n  \
      interfaces: {}, fields: {}, methods: {}, attributes: {}\
      ",
        this_class_access_flags,
        this_class_name,
        class_file.minor_version,
        class_file.major_version,
        class_file.access_flags,
        class_access_flags,
        this_class,
        get_constant_class_name(class_file.this_class, &class_file.cp_info),
        super_class,
        get_constant_class_name(class_file.super_class, &class_file.cp_info),
        class_file.interfaces_count,
        class_file.fields_count,
        class_file.methods_count,
        class_file.attributes_count,
    );

    println!("Constant pool:");
    for (idx, _item) in class_file.cp_info.iter().enumerate() {
        if idx != 0 {
            let line = cp_info_to_string(idx, &class_file.cp_info);
            println!("{}", line);
        }
    }

    /*
    // TMP debugging
    println!();
    for method_info in class_file.methods.iter() {
        println!("{:?}", method_info);
    }
    println!();
    */

    println!("{{");
    let mut it = class_file.methods.iter().peekable();
    while let Some(method_info) = it.next() {
        let method_str = method_info_to_string(method_info, &class_file);
        if it.peek().is_none() {
            println!("  {}", method_str);
        } else {
            println!("  {}\n", method_str);
        }
    }
    println!("}}");
}

fn cp_info_to_string(idx: usize, cp_info: &Vec<CpInfo>) -> String {
    match &cp_info[idx] {
        CpInfo::ConstantClass { tag: _tag, name_index } => {
            let idx_prefix = cp_info_index_prefix(idx);
            format!("{} = {:19}#{:<14}// {}", idx_prefix, "Class", name_index, get_constant_utf8(name_index.clone(), cp_info))
        },
        CpInfo::ConstantFieldref { tag: _tag, class_index, name_and_type_index } => {
            let idx_prefix = cp_info_index_prefix(idx);
            let class_name = get_constant_class_name(class_index.clone(), cp_info);
            let field_name = get_name(name_and_type_index.clone(), cp_info);
            let field_type = get_type(name_and_type_index.clone(), cp_info);
            let field_indexes = format!("#{}.#{}", class_index, name_and_type_index);
            format!("{} = {:19}{:15}// {}.{}:{}", idx_prefix, "Fieldref", field_indexes, class_name, field_name, field_type)
        },
        CpInfo::ConstantMethodref { tag: _tag, class_index, name_and_type_index } => {
            let idx_prefix = cp_info_index_prefix(idx);
            let class_name = get_constant_class_name(class_index.clone(), cp_info);
            let method_name = get_name_quoted(get_name(name_and_type_index.clone(), cp_info));
            let method_type = get_type(name_and_type_index.clone(), cp_info);
            let method_indexes = format!("#{}.#{}", class_index, name_and_type_index);
            format!("{} = {:19}{:15}// {}.{}:{}", idx_prefix, "Methodref", method_indexes, class_name, method_name, method_type)
        },
        CpInfo::ConstantString { tag: _tag, string_index } => {
            let idx_prefix = cp_info_index_prefix(idx);
            format!("{} = {:19}#{:<14}// {}", idx_prefix, "String", string_index, get_constant_utf8(string_index.clone(), cp_info))
        },
        CpInfo::ConstantNameAndType { tag: _tag, name_index, descriptor_index } => {
            let idx_prefix = cp_info_index_prefix(idx);
            let name = get_name_quoted(get_constant_utf8(name_index.clone(), cp_info));
            let typename = get_constant_utf8(descriptor_index.clone(), cp_info);
            let name_and_type_indexes = format!("#{}:#{}", name_index, descriptor_index);
            format!("{} = {:19}{:15}// {}:{}", idx_prefix, "NameAndType", name_and_type_indexes, name, typename)
        },
        CpInfo::ConstantUtf8 { tag: _tag, length: _length, bytes: _bytes, bytes_str } => {
            let idx_prefix = cp_info_index_prefix(idx);
            format!("{} = {:19}{}", idx_prefix, "Utf8", bytes_str)
        }
    }
}

fn cp_info_index_prefix(idx: usize) -> String {
    let left_pad = 5 - idx.to_string().len() - 1;
    format!("{0:<1$}#{2}", " ", left_pad, idx)
}

fn method_info_to_string(method_info: &MethodInfo, class_file: &ClassFile) -> String {
    let access_flags: Vec<MethodAccessFlag> = MethodAccessFlag::parse_flags(method_info.access_flags);
    let access_flags_java: Vec<&str> = access_flags.iter().map(|f| f.to_java_code()).collect();
    let access_flags_java: String = access_flags_java.join(" ");
    let access_flags_jvm: Vec<&str> = access_flags.iter().map(|f| f.to_str()).collect();
    let access_flags_jvm: String = access_flags_jvm.join(", ");
    let method_name = get_constant_utf8(method_info.name_index, &class_file.cp_info);
    let method_return_type: String = if method_name == "<init>" { String::from(" ") } else {
        let return_type = return_descriptor_to_java_code(method_info_return_type(method_info.descriptor_index, class_file));
        format!(" {} ", return_type)
    };
    let method_name: String = if method_name == "<init>" { get_constant_class_name(class_file.this_class, &class_file.cp_info) } else { method_name };
    let attributes = method_info_attributes(method_info, class_file);
    let arguments = parse_method_arguments(method_info, &class_file.cp_info);
    let arguments = parse_field_types(&arguments);
    let arguments: Vec<String> = arguments.iter().map(|f| f.str_java()).collect();
    let descriptor = get_constant_utf8(method_info.descriptor_index, &class_file.cp_info);

    format!("{}{}{}({});\n    \
        descriptor: {}\n    \
        flags: ({:#06x}) {}\n\
        {}",
        access_flags_java,
        method_return_type,
        method_name,
        arguments.join(", "),
        descriptor,
        method_info.access_flags,
        access_flags_jvm,
        attributes.join("\n")
    )
}

fn method_info_attributes(method_info: &MethodInfo, class_file: &ClassFile) -> Vec<String> {
    let mut attributes: Vec<String> = Vec::with_capacity(method_info.attributes_count as usize);
    for attribute in method_info.attributes.iter() {
        attributes.push(method_info_attribute(attribute, method_info, class_file));
    }
    attributes
}

fn method_info_attribute(attribute_info: &AttributeInfo, method_info: &MethodInfo, class_file: &ClassFile) -> String {
    match attribute_info {
        AttributeInfo::ConstantValue { .. } => todo!(),
        code @ AttributeInfo::Code { .. } => method_attribute_info_code(code, method_info, class_file),
        AttributeInfo::StackMapTable { .. } => todo!(),
        AttributeInfo::Exceptions { .. } => todo!(),
        AttributeInfo::InnerClasses { .. } => todo!(),
        AttributeInfo::EnclosingMethod { .. } => todo!(),
        AttributeInfo::Synthetic { .. } => todo!(),
        AttributeInfo::Signature { .. } => todo!(),
        AttributeInfo::SourceFile { .. } => todo!(),
        AttributeInfo::SourceDebugExtension { .. } => todo!(),
        lnt @ AttributeInfo::LineNumberTable { .. } => line_number_table(lnt),
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

fn method_attribute_info_code(code: &AttributeInfo, method_info: &MethodInfo, class_file: &ClassFile) -> String {
    if let AttributeInfo::Code {
        attribute_name_index: _attribute_name_index,
        attribute_length: _attribute_length,
        max_stack,
        max_locals,
        code_length: _code_length,
        code,
        exception_table_length: _exception_table_length,
        exception_table: _exception_table,
        attributes_count,
        attributes
    } = code {
        let instructions_block = instructions_block(code, class_file);
        let args_size = method_arguments_count(method_info, &class_file.cp_info);
        let code_attributes_len = attributes_count.clone() as usize;
        let mut code_attributes: Vec<String> = Vec::with_capacity(code_attributes_len);
        for i in 0..code_attributes_len {
            let attribute_str = method_info_attribute(&attributes[i], method_info, class_file);
            code_attributes.push(attribute_str);
        }

        format!(
            "    Code:\n      \
            stack={}, locals={}, args_size={}\n\
            {}\n{}",
            max_stack,
            max_locals,
            args_size,
            instructions_block,
            code_attributes.join("\n")
        )
    } else {
        panic!("Expected AttributeInfo::Code")
    }
}

fn line_number_table(line_number_table: &AttributeInfo) -> String {
    if let AttributeInfo::LineNumberTable {
        attribute_name_index: _attribute_name_index,
        attribute_length: _attribute_length,
        line_number_table_length,
        line_number_tables
    } = line_number_table {

        let mut items: Vec<String> = Vec::with_capacity(line_number_table_length.clone() as usize);
        for item in line_number_tables.iter() {
            let line = format!("        line {}: {}", item.line_number, item.start_pc);
            items.push(line);
        }

        format!("      LineNumberTable:\n{}", items.join("\n"))
    } else {
        panic!("Expected AttributeInfo::LineNumberTable")
    }
}

fn instructions_block(code: &Vec<u8>, class_file: &ClassFile) -> String {
    let mut acc: Vec<String> = Vec::new();
    let mut i: usize = 0;
    while i < code.len() {
        let opcode = get_opcode(code[i]);
        let (new_i, opcode_args_string_opt) = instruction_args(i, &opcode, code, class_file);
        let line = match opcode_args_string_opt {
            None => format!("{:10}: {}", i, opcode.str()),
            Some(opcode_args_string) => format!("{:10}: {:14}{}", i, opcode.str(), opcode_args_string)
        };
        acc.push(line);
        i = new_i;
    }
    acc.join("\n")
}

fn instruction_args(opcode_idx: usize, opcode: &Opcode, code: &Vec<u8>, class_file: &ClassFile) -> (usize, Option<String>) {
    match opcode {
        Opcode::Nop => (opcode_idx + 1, None),
        Opcode::AconstNull => todo!("opcode_idx[{}]", opcode_idx),
        Opcode::IconstM1 => todo!(),
        Opcode::Iconst0 => todo!(),
        Opcode::Iconst1 => todo!(),
        Opcode::Iconst2 => todo!(),
        Opcode::Iconst3 => todo!(),
        Opcode::Iconst4 => todo!(),
        Opcode::Iconst5 => todo!(),
        Opcode::Lconst0 => todo!(),
        Opcode::Lconst1 => todo!(),
        Opcode::Fconst0 => todo!(),
        Opcode::Fconst1 => todo!(),
        Opcode::Fconst2 => todo!(),
        Opcode::Dconst0 => todo!(),
        Opcode::Dconst1 => todo!("opcode_idx[{}] in {:?}", opcode_idx, code),
        Opcode::Bipush => todo!(),
        Opcode::Sipush => todo!(),
        Opcode::Ldc => {
            // Push item from run-time constant pool
            let (new_idx, index) = get_u1(opcode_idx + 1, code);
            let description = get_ldc_description(index as usize, class_file);
            (new_idx, Some(format!("#{:<19}// {}", index, description)))
        },
        Opcode::LdcW => todo!(),
        Opcode::Ldc2W => todo!(),
        Opcode::Iload => todo!(),
        Opcode::Lload => todo!(),
        Opcode::Fload => todo!(),
        Opcode::Dload => todo!(),
        Opcode::Aload => todo!(),
        Opcode::Iload0 => todo!(),
        Opcode::Iload1 => todo!(),
        Opcode::Iload2 => todo!(),
        Opcode::Iload3 => todo!(),
        Opcode::Lload0 => todo!(),
        Opcode::Lload1 => todo!(),
        Opcode::Lload2 => todo!(),
        Opcode::Lload3 => todo!(),
        Opcode::Fload0 => todo!(),
        Opcode::Fload1 => todo!(),
        Opcode::Fload2 => todo!(),
        Opcode::Fload3 => todo!(),
        Opcode::Dload0 => todo!(),
        Opcode::Dload1 => todo!(),
        Opcode::Dload2 => todo!(),
        Opcode::Dload3 => todo!(),
        Opcode::Aload0 => {
            // Load reference from local variable
            (opcode_idx + 1, None)
        },
        Opcode::Aload1 => todo!(),
        Opcode::Aload2 => todo!(),
        Opcode::Aload3 => todo!(),
        Opcode::Iaload => todo!(),
        Opcode::Laload => todo!(),
        Opcode::Faload => todo!(),
        Opcode::Daload => todo!(),
        Opcode::Aaload => todo!(),
        Opcode::Baload => todo!(),
        Opcode::Caload => todo!(),
        Opcode::Saload => todo!(),
        Opcode::Istore => todo!(),
        Opcode::Lstore => todo!(),
        Opcode::Fstore => todo!(),
        Opcode::Dstore => todo!(),
        Opcode::Astore => todo!(),
        Opcode::Istore0 => todo!(),
        Opcode::Istore1 => todo!(),
        Opcode::Istore2 => todo!(),
        Opcode::Istore3 => todo!(),
        Opcode::Lstore0 => todo!(),
        Opcode::Lstore1 => todo!(),
        Opcode::Lstore2 => todo!(),
        Opcode::Lstore3 => todo!(),
        Opcode::Fstore0 => todo!(),
        Opcode::Fstore1 => todo!(),
        Opcode::Fstore2 => todo!(),
        Opcode::Fstore3 => todo!(),
        Opcode::Dstore0 => todo!(),
        Opcode::Dstore1 => todo!(),
        Opcode::Dstore2 => todo!(),
        Opcode::Dstore3 => todo!(),
        Opcode::Astore0 => todo!(),
        Opcode::Astore1 => todo!(),
        Opcode::Astore2 => todo!(),
        Opcode::Astore3 => todo!(),
        Opcode::Iastore => todo!(),
        Opcode::Lastore => todo!(),
        Opcode::Fastore => todo!(),
        Opcode::Dastore => todo!(),
        Opcode::Aastore => todo!(),
        Opcode::Bastore => todo!(),
        Opcode::Castore => todo!(),
        Opcode::Sastore => todo!(),
        Opcode::Pop => todo!(),
        Opcode::Pop2 => todo!(),
        Opcode::Dup => todo!(),
        Opcode::DupX1 => todo!(),
        Opcode::DupX2 => todo!(),
        Opcode::Dup2 => todo!(),
        Opcode::Dup2X1 => todo!(),
        Opcode::Dup2X2 => todo!(),
        Opcode::Swap => todo!(),
        Opcode::Iadd => todo!(),
        Opcode::Ladd => todo!(),
        Opcode::Fadd => todo!(),
        Opcode::Dadd => todo!(),
        Opcode::Isub => todo!(),
        Opcode::Lsub => todo!(),
        Opcode::Fsub => todo!(),
        Opcode::Dsub => todo!(),
        Opcode::Imul => todo!(),
        Opcode::Lmul => todo!(),
        Opcode::Fmul => todo!(),
        Opcode::Dmul => todo!(),
        Opcode::Idiv => todo!(),
        Opcode::Ldiv => todo!(),
        Opcode::Fdiv => todo!(),
        Opcode::Ddiv => todo!(),
        Opcode::Irem => todo!(),
        Opcode::Lrem => todo!(),
        Opcode::Frem => todo!(),
        Opcode::Drem => todo!(),
        Opcode::Ineg => todo!(),
        Opcode::Lneg => todo!(),
        Opcode::Fneg => todo!(),
        Opcode::Dneg => todo!(),
        Opcode::Ishl => todo!(),
        Opcode::Lshl => todo!(),
        Opcode::Ishr => todo!(),
        Opcode::Lshr => todo!(),
        Opcode::Iushr => todo!(),
        Opcode::Lushr => todo!(),
        Opcode::Iand => todo!(),
        Opcode::Land => todo!(),
        Opcode::Ior => todo!(),
        Opcode::Lor => todo!(),
        Opcode::Ixor => todo!(),
        Opcode::Lxor => todo!(),
        Opcode::Iinc => todo!(),
        Opcode::I2l => todo!(),
        Opcode::I2f => todo!(),
        Opcode::I2d => todo!(),
        Opcode::L2i => todo!(),
        Opcode::L2f => todo!(),
        Opcode::L2d => todo!(),
        Opcode::F2i => todo!(),
        Opcode::F2l => todo!(),
        Opcode::F2d => todo!(),
        Opcode::D2i => todo!(),
        Opcode::D2l => todo!(),
        Opcode::D2f => todo!(),
        Opcode::I2b => todo!(),
        Opcode::I2c => todo!(),
        Opcode::I2s => todo!(),
        Opcode::Lcmp => todo!(),
        Opcode::Fcmpl => todo!(),
        Opcode::Fcmpg => todo!(),
        Opcode::Dcmpl => todo!(),
        Opcode::Dcmpg => todo!(),
        Opcode::Ifeq => todo!(),
        Opcode::Ifne => todo!(),
        Opcode::Iflt => todo!(),
        Opcode::Ifge => todo!(),
        Opcode::Ifgt => todo!(),
        Opcode::Ifle => todo!(),
        Opcode::IfIcmpeq => todo!(),
        Opcode::IfIcmpne => todo!(),
        Opcode::IfIcmplt => todo!(),
        Opcode::IfIcmpge => todo!(),
        Opcode::IfIcmpgt => todo!(),
        Opcode::IfIcmple => todo!(),
        Opcode::IfAcmpeq => todo!(),
        Opcode::IfAcmpne => todo!(),
        Opcode::Goto => todo!(),
        Opcode::Jsr => todo!(),
        Opcode::Ret => todo!(),
        Opcode::Tableswitch => todo!(),
        Opcode::Lookupswitch => todo!(),
        Opcode::Ireturn => todo!(),
        Opcode::Lreturn => todo!(),
        Opcode::Freturn => todo!(),
        Opcode::Dreturn => todo!(),
        Opcode::Areturn => todo!(),
        Opcode::Return => {
            // Return void from method
            (opcode_idx + 1, None)
        },
        Opcode::Getstatic => {
            // get static field from a class
            let (new_idx, fieldref) = get_u2(opcode_idx + 1, code);
            let description = get_static_description(fieldref as usize, class_file);
            (new_idx, Some(format!("#{:<19}// {}", fieldref, description)))
        },
        Opcode::Putstatic => todo!(),
        Opcode::Getfield => todo!(),
        Opcode::Putfield => todo!(),
        Opcode::Invokevirtual => {
            // Invoke instance method; dispatch based on class
            let (new_idx, index) = get_u2(opcode_idx + 1, code);
            let method = get_constant_method_ref_description(index as usize, class_file);
            (new_idx, Some(format!("#{:<19}// Method {}", index, method)))
        },
        Opcode::Invokespecial => {
            // Invoke instance method; direct invocation of instance initialization methods and methods of the current class and its supertypes
            let (new_idx, index) = get_u2(opcode_idx + 1, code);
            let method = get_constant_method_ref_description(index as usize, class_file);
            (new_idx, Some(format!("#{:<19}// Method {}", index, method)))
        },
        Opcode::Invokestatic => todo!(),
        Opcode::Invokeinterface => todo!(),
        Opcode::Invokedynamic => todo!(),
        Opcode::New => todo!(),
        Opcode::Newarray => todo!(),
        Opcode::Anewarray => todo!(),
        Opcode::Arraylength => todo!(),
        Opcode::Athrow => todo!(),
        Opcode::Checkcast => todo!(),
        Opcode::Instanceof => todo!(),
        Opcode::Monitorenter => todo!(),
        Opcode::Monitorexit => todo!(),
        Opcode::Wide => todo!(),
        Opcode::Multianewarray => todo!(),
        Opcode::Ifnull => todo!(),
        Opcode::Ifnonnull => todo!(),
        Opcode::GotoW => todo!(),
        Opcode::JsrW => todo!(),
        Opcode::Breakpoint => todo!(),
        Opcode::Impdep1 => todo!(),
        Opcode::Impdep2 => todo!(),
    }
}

