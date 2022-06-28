#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub cp_info: Vec<CpInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
}

#[derive(Debug)]
pub enum CpInfo {
    ConstantClass { tag: u8, name_index: u16 },
    ConstantFieldref { tag: u8, class_index: u16, name_and_type_index: u16 },
    ConstantMethodref { tag: u8, class_index: u16, name_and_type_index: u16 },
    // ConstantInterfaceMethodref
    ConstantString { tag: u8, string_index: u16 },
    // ConstantInteger
    // ConstantFloat
    // ConstantLong
    // ConstantDouble
    ConstantNameAndType { tag: u8, name_index: u16, descriptor_index: u16 },
    ConstantUtf8 { tag: u8, length: u16, bytes: Vec<u8>, bytes_str: String },
    // ConstantMethodHandle
    // ConstantMethodType
    // ConstantDynamic
    // ConstantInvokeDynamic
    // ConstantModule
    // ConstantPackage
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>
}

#[derive(Debug)]
pub enum AttributeInfo {
    ConstantValue { attribute_name_index: u16, attribute_length: u32, constantvalue_index: u16 },
    // TODO:
    Code {},
    StackMapTable {},
    Exceptions {},
    InnerClasses {},
    EnclosingMethod {},
    Synthetic {},
    Signature {},
    SourceFile {},
    SourceDebugExtension {},
    LineNumberTable {},
    LocalVariableTable {},
    LocalVariableTypeTable {},
    Deprecated {},
    RuntimeVisibleAnnotations {},
    RuntimeInvisibleAnnotations {},
    RuntimeVisibleParameterAnnotations {},
    RuntimeInvisibleParameterAnnotations {},
    RuntimeVisibleTypeAnnotations {},
    RuntimeInvisibleTypeAnnotations {},
    AnnotationDefault {},
    BootstrapMethods {},
    MethodParameters {},
    Module {},
    ModulePackages {},
    ModuleMainClass {},
    NestHost {},
    NestMembers {},
    Record {},
    PermittedSubclasses {},
}