use crate::ast::ClassFile;

pub fn pretty_print_text(class_file: &ClassFile) {
    println!("TODO: public class A");
    println!("  minor:{:x}\n  major:{}\n  flags:{:x}\n  this_class: #{}\n  super_class: #{}\n  interfaces: {}, fields: TODO, methods: TODO, attributes: TODO",
        class_file.minor_version,
        class_file.major_version,
        class_file.access_flags,
        class_file.this_class,
        class_file.super_class,
        class_file.interfaces_count,
    );

    println!("Constant pool({}):", class_file.constant_pool_count);

    println!("interfaces:{:?} fields_count:{}",
        class_file.interfaces,
        class_file.fields_count,
    );

    for (idx, item) in class_file.cp_info.iter().enumerate() {
        println!("item[{}]:{:?}", idx, item);
    }
}