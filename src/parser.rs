use crate::ast::{AttributeInfo, ClassFile, CpInfo, FieldInfo, MethodInfo};

pub fn parse_class_file(bytecode: &Vec<u8>) -> ClassFile {
    let idx: usize = 0;
    let (idx, magic) = get_u4(idx, bytecode);
    let (idx, minor_version) = get_u2(idx, bytecode);
    let (idx, major_version) = get_u2(idx, bytecode);
    let (idx, constant_pool_count) = get_u2(idx, bytecode);
    let (idx, cp_info) = parse_cp_info_array(idx, constant_pool_count, bytecode);
    let (idx, access_flags) = get_u2(idx, bytecode);
    let (idx, this_class) = get_u2(idx, bytecode);
    let (idx, super_class) = get_u2(idx, bytecode);
    let (idx, interfaces_count) = get_u2(idx, bytecode);
    let (idx, interfaces) = parse_interfaces(idx, interfaces_count, bytecode);
    let (idx, fields_count) = get_u2(idx, bytecode);
    let (idx, fields) = parse_fields(idx, fields_count, bytecode);
    let (idx, methods_count) = get_u2(idx, bytecode);
    let (idx, methods) = parse_methods(idx, methods_count, bytecode);

    ClassFile {
        magic,
        minor_version,
        major_version,
        constant_pool_count,
        cp_info,
        access_flags,
        this_class,
        super_class,
        interfaces_count,
        interfaces,
        fields_count,
        fields,
        methods_count,
        methods,
    }
}

fn parse_cp_info_array(idx: usize, constant_pool_count: u16, bytecode: &Vec<u8>) -> (usize, Vec<CpInfo>) {
    let mut cp_infos: Vec<CpInfo> = Vec::new();
    let dummy = CpInfo::ConstantUtf8 { tag: 0, length: 0, bytes: vec![], bytes_str: String::from("Dummy Value") };
    cp_infos.push(dummy);
    let mut idx = idx;
    for _ in 1..constant_pool_count {
        let (new_idx, cp_info) = parse_cp_info(idx, bytecode);
        idx = new_idx;
        cp_infos.push(cp_info);
    }
    (idx, cp_infos)
}

fn parse_cp_info(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, tag) = get_u1(idx, bytecode);
    match tag {
        7 => parse_constant_class(idx, bytecode),
        9 => parse_constant_fieldref(idx, bytecode),
        10 => parse_constant_methodref(idx, bytecode),
        11 => todo!("CONSTANT_InterfaceMethodref"), // CONSTANT_InterfaceMethodref
        8 => parse_constant_string(idx, bytecode),
        3 => todo!("CONSTANT_Integer"), // CONSTANT_Integer
        4 => todo!("CONSTANT_Float"), // CONSTANT_Float
        5 => todo!("CONSTANT_Long"), // CONSTANT_Long
        6 => todo!(), // CONSTANT_Double
        12 => parse_constant_name_and_type(idx, bytecode),
        1 => parse_constant_utf8(idx, bytecode),
        15 => todo!(), // CONSTANT_MethodHandle
        16 => todo!(), // CONSTANT_MethodType
        17 => todo!(), // CONSTANT_Dynamic
        18 => todo!(), // CONSTANT_InvokeDynamic
        19 => todo!(), // CONSTANT_Module
        20 => todo!(), // CONSTANT_Package
        _ => todo!() // fail hard
    }
}

/// 7 CONSTANT_Class
fn parse_constant_class(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, name_index) = get_u2(idx, bytecode);
    (idx, CpInfo::ConstantClass { tag: 7, name_index })
}

/// 9 CONSTANT_Fieldref
fn parse_constant_fieldref(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, class_index) = get_u2(idx, bytecode);
    let (idx, name_and_type_index) = get_u2(idx, bytecode);
    (idx, CpInfo::ConstantFieldref { tag: 9, class_index, name_and_type_index })
}

/// 10 CONSTANT_Methodref
fn parse_constant_methodref(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, class_index) = get_u2(idx, bytecode);
    let (idx, name_and_type_index) = get_u2(idx, bytecode);
    (idx, CpInfo::ConstantMethodref { tag: 10, class_index, name_and_type_index })
}

/// 8 CONSTANT_String
fn parse_constant_string(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, string_index) = get_u2(idx, bytecode);
    (idx, CpInfo::ConstantString { tag: 8, string_index })
}

/// 12 CONSTANT_NameAndType
fn parse_constant_name_and_type(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, name_index) = get_u2(idx, bytecode);
    let (idx, descriptor_index) = get_u2(idx, bytecode);
    (idx, CpInfo::ConstantNameAndType { tag: 12, name_index, descriptor_index })
}

/// 1 CONSTANT_Utf8
fn parse_constant_utf8(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, length) = get_u2(idx, bytecode);
    let len = length as usize;
    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    for i in 0..len {
        bytes.push(bytecode[idx + i])
    }
    let bytes_str: String = String::from_utf8(bytes.clone()).expect("Invalid UTF-8");
    (idx + len, CpInfo::ConstantUtf8 { tag: 1, length, bytes, bytes_str })
}

fn parse_interfaces(idx: usize, interfaces_count: u16, bytecode: &Vec<u8>) -> (usize, Vec<u16>) {
    let count = interfaces_count as usize;
    let mut v: Vec<u16> = Vec::with_capacity(count);
    let mut idx = idx;
    for _ in 0..count {
        let (i, interface) = get_u2(idx, bytecode);
        idx = i;
        v.push(interface);
    }
    (idx, v)
}

fn parse_fields(idx: usize, fields_count: u16, bytecode: &Vec<u8>) -> (usize, Vec<FieldInfo>) {
    let count = fields_count as usize;
    let mut idx = idx;
    let mut fields: Vec<FieldInfo> = Vec::with_capacity(count);
    for _ in 0..count {
        let (i, field_info) = parse_field_info(idx, bytecode);
        idx = i;
        fields.push(field_info);
    }
    (idx, fields)
}

fn parse_field_info(idx: usize, bytecode: &Vec<u8>) -> (usize, FieldInfo) {
    todo!()
}

fn parse_methods(idx: usize, methods_count: u16, bytecode: &Vec<u8>) -> (usize, Vec<MethodInfo>) {
    let mut idx = idx;
    let count = methods_count as usize;
    let mut attributes: Vec<MethodInfo> = Vec::with_capacity(count);
    for _ in 0..count {
        let (i, method_info) = parse_method_info(idx, &bytecode);
        idx = i;
        attributes.push(method_info);
    }
    (idx, attributes)
}

fn parse_method_info(idx: usize, bytecode: &Vec<u8>) -> (usize, MethodInfo) {
    let (idx, access_flags) = get_u2(idx, bytecode);
    let (idx, name_index) = get_u2(idx, bytecode);
    let (idx, descriptor_index) = get_u2(idx, bytecode);
    let (idx, attributes_count) = get_u2(idx, bytecode);
    let (idx, attributes) = parse_attributes(idx, attributes_count, bytecode);
    let method_info = MethodInfo { access_flags, name_index, descriptor_index, attributes_count, attributes };
    (idx, method_info)
}

fn parse_attributes(idx: usize, attributes_count: u16, bytecode: &Vec<u8>) -> (usize, Vec<AttributeInfo>) {
    let mut idx = idx;
    let count = attributes_count as usize;
    let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(count);
    for _ in 0..count {
        let (i, attribute_info) = parse_attribute_info(idx, bytecode);
        idx = i;
        attributes.push(attribute_info);
    }
    (idx, attributes)
}

fn parse_attribute_info(idx: usize, bytecode: &Vec<u8>) -> (usize, AttributeInfo) {
    let (idx, attribute_name_index) = get_u2(idx, bytecode);
    let (idx, attribute_length) = get_u4(idx, bytecode);
    todo!("We need to lookup the constant pool to decide which AttributeInfo to parse")
}

fn get_u1(idx: usize, bytecode: &Vec<u8>) -> (usize, u8) {
    (idx + 1, bytecode[idx])
}

fn get_u2(idx: usize, bytecode: &Vec<u8>) -> (usize, u16) {
    let u0 = (bytecode[idx] as u16) << 8;
    let u1 = bytecode[idx + 1] as u16;
    let r: u16 = u0 | u1;
    (idx + 2, r)
}

fn get_u4(idx: usize, bytecode: &Vec<u8>) -> (usize, u32) {
    let u0 = (bytecode[idx] as u32) << 24;
    let u1 = (bytecode[idx + 1] as u32) << 16;
    let u2 = (bytecode[idx + 2] as u32) << 8;
    let u3 = bytecode[idx + 3] as u32;
    let r: u32 = u0 | u1 | u2 | u3;
    (idx + 4, r)
}

