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

pub fn get_method_access_flags(flags: u16) -> Vec<String> {
    let flag_to_description: HashMap<u16, &str> = HashMap::from([
        (0x0001, "ACC_PUBLIC"),
        (0x0002, "ACC_PRIVATE"),
        (0x0004, "ACC_PROTECTED"),
        (0x0008, "ACC_STATIC"),
        (0x0010, "ACC_FINAL"),
        (0x0020, "ACC_SYNCHRONIZED"),
        (0x0040, "ACC_BRIDGE"),
        (0x0080, "ACC_VARARGS"),
        (0x0100, "ACC_NATIVE"),
        (0x0400, "ACC_ABSTRACT"),
        (0x0800, "ACC_STRICT"),
        (0x1000, "ACC_SYNTHETIC")
    ]);
    let mut descriptions: Vec<String> = Vec::new();
    for (f, v) in flag_to_description {
        if f & flags != 0 {
            descriptions.push(String::from(v))
        }
    }
    descriptions.sort();
    descriptions
}