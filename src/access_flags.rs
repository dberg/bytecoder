use std::collections::HashMap;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum ClassAccessFlag {
    AccPublic,
    AccFinal,
    AccSuper,
    AccInterface,
    AccAbstract,
    AccSynthetic,
    AccAnnotation,
    AccEnum,
    AccModule,
}

impl ClassAccessFlag {
    pub fn to_str(&self) -> &str {
        match self {
            ClassAccessFlag::AccPublic => "ACC_PUBLIC",
            ClassAccessFlag::AccFinal => "ACC_FINAL",
            ClassAccessFlag::AccSuper => "ACC_SUPER",
            ClassAccessFlag::AccInterface => "ACC_INTERFACE",
            ClassAccessFlag::AccAbstract => "ACC_ABSTRACT",
            ClassAccessFlag::AccSynthetic => "ACC_SYNTHETIC",
            ClassAccessFlag::AccAnnotation => "ACC_ANNOTATION",
            ClassAccessFlag::AccEnum => "ACC_ENUM",
            ClassAccessFlag::AccModule => "ACC_MODULE"
        }
    }

    pub fn to_java_code(&self) -> &str {
        match self {
            ClassAccessFlag::AccPublic => "public",
            ClassAccessFlag::AccFinal => "final",
            ClassAccessFlag::AccSuper => "super",
            ClassAccessFlag::AccInterface => "interface",
            ClassAccessFlag::AccAbstract => "abstract",
            ClassAccessFlag::AccSynthetic => "synthetic",
            ClassAccessFlag::AccAnnotation => "annotation",
            ClassAccessFlag::AccEnum => "enum",
            ClassAccessFlag::AccModule => "module"
        }
    }

    pub fn parse_flags(flags: u16) -> Vec<ClassAccessFlag> {
        let flag_to_class_access_flag: HashMap<u16, ClassAccessFlag> = HashMap::from([
            (0x0001, ClassAccessFlag::AccPublic),
            (0x0010, ClassAccessFlag::AccFinal),
            (0x0020, ClassAccessFlag::AccSuper),
            (0x0200, ClassAccessFlag::AccInterface),
            (0x0400, ClassAccessFlag::AccAbstract),
            (0x1000, ClassAccessFlag::AccSynthetic),
            (0x2000, ClassAccessFlag::AccAnnotation),
            (0x4000, ClassAccessFlag::AccEnum),
            (0x8000, ClassAccessFlag::AccModule),
        ]);

        let mut descriptions: Vec<ClassAccessFlag> = Vec::new();
        for (f, v) in flag_to_class_access_flag {
            if f & flags != 0 {
                descriptions.push(v);
            }
        }
        descriptions.sort();
        descriptions
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum MethodAccessFlag {
    AccPublic,
    AccPrivate,
    AccProtected,
    AccStatic,
    AccFinal,
    AccSynchronized,
    AccBridge,
    AccVarargs,
    AccNative,
    AccAbstract,
    AccStrict,
    AccSynthetic
}

impl MethodAccessFlag {
    pub fn to_str(&self) -> &str {
        match self {
            MethodAccessFlag::AccPublic => "ACC_PUBLIC",
            MethodAccessFlag::AccPrivate => "ACC_PRIVATE",
            MethodAccessFlag::AccProtected => "ACC_PROTECTED",
            MethodAccessFlag::AccStatic => "ACC_STATIC",
            MethodAccessFlag::AccFinal => "ACC_FINAL",
            MethodAccessFlag::AccSynchronized => "ACC_SYNCHRONIZED",
            MethodAccessFlag::AccBridge => "ACC_BRIDGE",
            MethodAccessFlag::AccVarargs => "ACC_VARARGS",
            MethodAccessFlag::AccNative => "ACC_NATIVE",
            MethodAccessFlag::AccAbstract => "ACC_ABSTRACT",
            MethodAccessFlag::AccStrict => "ACC_STRICT",
            MethodAccessFlag::AccSynthetic => "ACC_SYNTHETIC"
        }
    }

    pub fn to_java_code(&self) -> &str {
        match self {
            MethodAccessFlag::AccPublic => "public",
            MethodAccessFlag::AccPrivate => "private",
            MethodAccessFlag::AccProtected => "protected",
            MethodAccessFlag::AccStatic => "static",
            MethodAccessFlag::AccFinal => "final",
            MethodAccessFlag::AccSynchronized => "synchronized",
            MethodAccessFlag::AccBridge => "bridge",
            MethodAccessFlag::AccVarargs => "varargs",
            MethodAccessFlag::AccNative => "native",
            MethodAccessFlag::AccAbstract => "abstract",
            MethodAccessFlag::AccStrict => "strict",
            MethodAccessFlag::AccSynthetic => "synthetic"
        }
    }

    pub fn parse_flags(flags: u16) -> Vec<MethodAccessFlag> {
        let flag_to_method_access_flag: HashMap<u16, MethodAccessFlag> = HashMap::from([
            (0x0001, MethodAccessFlag::AccPublic),
            (0x0002, MethodAccessFlag::AccPrivate),
            (0x0004, MethodAccessFlag::AccProtected),
            (0x0008, MethodAccessFlag::AccStatic),
            (0x0010, MethodAccessFlag::AccFinal),
            (0x0020, MethodAccessFlag::AccSynchronized),
            (0x0040, MethodAccessFlag::AccBridge),
            (0x0080, MethodAccessFlag::AccVarargs),
            (0x0100, MethodAccessFlag::AccNative),
            (0x0400, MethodAccessFlag::AccAbstract),
            (0x0800, MethodAccessFlag::AccStrict),
            (0x1000, MethodAccessFlag::AccSynthetic)
        ]);

        let mut descriptions: Vec<MethodAccessFlag> = Vec::new();
        for (f, v) in flag_to_method_access_flag {
            if f & flags != 0 {
                descriptions.push(v)
            }
        }
        descriptions.sort();
        descriptions
    }
}
