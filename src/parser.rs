use crate::ast::{ClassFile, CpInfo, FieldInfo};

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
    let (idx, fields) = parse_fields(idx, fields_count, &cp_info, bytecode);

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
        fields
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
    let c = interfaces_count as usize;
    let mut v: Vec<u16> = Vec::with_capacity(c);
    let mut idx = idx;
    for _ in 0..c {
        let (idx_new, interface) = get_u2(idx, bytecode);
        idx = idx_new;
        v.push(interface);
    }
    (idx, v)
}

fn parse_fields(idx: usize, fields_count: u16, cp_info: &Vec<CpInfo>, bytecode: &Vec<u8>) -> (usize, Vec<FieldInfo>) {
    todo!()
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

