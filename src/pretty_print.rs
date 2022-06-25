use crate::ast::ClassFile;

pub fn pretty_print_text(class_file: &ClassFile) {
    println!("The class file magic:{:x} minor:{:x} major:{} constant_pool_count:{}",
        class_file.magic,
        class_file.minor_version,
        class_file.major_version,
        class_file.constant_pool_count,
    );

    for (idx, item) in class_file.cp_info.iter().enumerate() {
        println!("item[{}]:{:?}", idx, item);
    }

    println!("access_flags:{:x} this_class:{} super_class:{} interfaces_count:{} interfaces:{:?} fields_count:{}",
        class_file.access_flags,
        class_file.this_class,
        class_file.super_class,
        class_file.interfaces_count,
        class_file.interfaces,
        class_file.fields_count,
    );
}