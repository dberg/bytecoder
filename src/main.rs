#[derive(Debug)]
struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    cp_info: Vec<CpInfo>,
}

#[derive(Debug)]
enum CpInfo {
    ConstantMethodref { tag: u8, class_index: u16, name_and_type_index: u16 },
}

fn main() {

    let hello_world: Vec<u8> = vec![
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x3e,
        0x00, 0x1d, 0x0a, 0x00, 0x02, 0x00, 0x03, 0x07,
        0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01,
        0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65,
        0x63, 0x74, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e,
        0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29,
        0x56, 0x09, 0x00, 0x08, 0x00, 0x09, 0x07, 0x00,
        0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00,
        0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61,
        0x6e, 0x67, 0x2f, 0x53, 0x79, 0x73, 0x74, 0x65,
        0x6d, 0x01, 0x00, 0x03, 0x6f, 0x75, 0x74, 0x01,
        0x00, 0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f,
        0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74,
        0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x08,
        0x00, 0x0e, 0x01, 0x00, 0x0d, 0x48, 0x65, 0x6c,
        0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c,
        0x64, 0x2e, 0x0a, 0x00, 0x10, 0x00, 0x11, 0x07,
        0x00, 0x12, 0x0c, 0x00, 0x13, 0x00, 0x14, 0x01,
        0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69,
        0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53,
        0x74, 0x72, 0x65, 0x61, 0x6d, 0x01, 0x00, 0x07,
        0x70, 0x72, 0x69, 0x6e, 0x74, 0x6c, 0x6e, 0x01,
        0x00, 0x15, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74,
        0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x07,
        0x00, 0x16, 0x01, 0x00, 0x01, 0x41, 0x01, 0x00,
        0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f,
        0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62,
        0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01,
        0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00,
        0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74,
        0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01,
        0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65,
        0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x06, 0x41,
        0x2e, 0x6a, 0x61, 0x76, 0x61, 0x00, 0x21, 0x00,
        0x15, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x01, 0x00, 0x05, 0x00, 0x06, 0x00,
        0x01, 0x00, 0x17, 0x00, 0x00, 0x00, 0x1d, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a,
        0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x18, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x09, 0x00, 0x19,
        0x00, 0x1a, 0x00, 0x01, 0x00, 0x17, 0x00, 0x00,
        0x00, 0x25, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x09, 0xb2, 0x00, 0x07, 0x12, 0x0d, 0xb6,
        0x00, 0x0f, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x18, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x03, 0x00, 0x08, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x1c
    ];
    let class_file = parse_class_file(&hello_world);
    println!("The class file magic:{:x} minor:{:x} major:{} constant_pool_count:{}",
        class_file.magic,
        class_file.minor_version,
        class_file.major_version,
        class_file.constant_pool_count,
    );

    for item in class_file.cp_info {
        println!("item:{:?}", item);
    }
}

fn parse_class_file(bytecode: &Vec<u8>) -> ClassFile {
    let idx: usize = 0;
    let (idx, magic) = get_u4(idx, bytecode);
    let (idx, minor_version) = get_u2(idx, bytecode);
    let (idx, major_version) = get_u2(idx, bytecode);
    let (idx, constant_pool_count) = get_u2(idx, bytecode);
    let (idx, cp_info) = parse_cp_info_array(idx, constant_pool_count, bytecode);
    ClassFile { magic, minor_version, major_version, constant_pool_count, cp_info }
}

fn parse_cp_info_array(idx: usize, constant_pool_count: u16, bytecode: &Vec<u8>) -> (usize, Vec<CpInfo>) {
    let mut cp_infos: Vec<CpInfo> = Vec::new();
    let mut idx = idx;
    for _ in 0..constant_pool_count {
        let (new_idx, cp_info) = parse_cp_info(idx, bytecode);
        idx = new_idx;
        cp_infos.push(cp_info);
    }
    (idx, cp_infos)
}

fn parse_cp_info(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, tag) = get_u1(idx, bytecode);
    match tag {
        7 => todo!(), // CONSTANT_Class
        9 => todo!(), // CONSTANT_Fieldref
        10 => parse_constant_methodref(idx, bytecode),
        11 => todo!(), // CONSTANT_InterfaceMethodref
        8 => todo!(), // CONSTANT_String
        3 => todo!(), // CONSTANT_Integer
        4 => todo!(), // CONSTANT_Float
        5 => todo!(), // CONSTANT_Long
        6 => todo!(), // CONSTANT_Double
        12 => todo!(), // CONSTANT_NameAndType
        1 => todo!(), // CONSTANT_Utf8
        15 => todo!(), // CONSTANT_MethodHandle
        16 => todo!(), // CONSTANT_MethodType
        17 => todo!(), // CONSTANT_Dynamic
        18 => todo!(), // CONSTANT_InvokeDynamic
        19 => todo!(), // CONSTANT_Module
        20 => todo!(), // CONSTANT_Package
        _ => todo!() // fail hard
    }
}

/// 10 CONSTANT_Methodref
fn parse_constant_methodref(idx: usize, bytecode: &Vec<u8>) -> (usize, CpInfo) {
    let (idx, class_index) = get_u2(idx, bytecode);
    let (idx, name_and_type_index) = get_u2(idx, bytecode);
    println!("CONSTANT_Methodref class_index:{}, name_and_type_index:{}", class_index, name_and_type_index);
    (idx, CpInfo::ConstantMethodref { tag: 10, class_index, name_and_type_index })
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
