use crate::ast::{AttributeInfo, ClassFile, CpInfo, MethodInfo};
use crate::opcodes::{get_opcode, Opcode};
use crate::parser::{get_class_access_flags, get_method_access_flags, get_u1, get_u2};

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
        let instructions_block = instructions_block(code);
        format!(
            "    Code:\n      \
            stack={}, locals={}, args_size=TODO\n\
            {}",
            max_stack,
            max_locals,
            instructions_block
        )
    } else {
        panic!("Expected AttributeInfo::Code")
    }
}

fn instructions_block(code: &Vec<u8>) -> String {
    let mut acc: Vec<String> = Vec::new();
    let mut i: usize = 0;
    while i < code.len() {
        let opcode = get_opcode(code[i]);
        let (new_i, opcode_args_string) = instruction_args(i, &opcode, code);
        let line = format!("         {}: {}\t{}", i, opcode.str(), opcode_args_string);
        acc.push(line);
        i = new_i;
    }
    acc.join("\n")
}

fn instruction_args(opcode_idx: usize, opcode: &Opcode, code: &Vec<u8>) -> (usize, String) {
    match opcode {
        Opcode::Nop => (opcode_idx + 1, String::from("")),
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
            (new_idx, format!("#{}", index))
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
            (opcode_idx + 1, String::from(""))
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
            (opcode_idx + 1, String::from(""))
        },
        Opcode::Getstatic => {
            // get static field from a class
            let (new_idx, fieldref) = get_u2(opcode_idx + 1, code);
            (new_idx, format!("#{}", fieldref))
        },
        Opcode::Putstatic => todo!(),
        Opcode::Getfield => todo!(),
        Opcode::Putfield => todo!(),
        Opcode::Invokevirtual => {
            // Invoke instance method; dispatch based on class
            let (new_idx, index) = get_u2(opcode_idx + 1, code);
            (new_idx, format!("#{}", index))
        },
        Opcode::Invokespecial => {
            // Invoke instance method; direct invocation of instance initialization methods and methods of the current class and its supertypes
            let (new_idx, index) = get_u2(opcode_idx + 1, code);
            (new_idx, format!("#{}", index))
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

