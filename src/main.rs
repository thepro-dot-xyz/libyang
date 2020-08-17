use libyang::*;

fn main() {
    // Allocate a new Yang.
    let mut yang = Yang::new();
    yang.add_path("/etc/openconfigd/yang:yang/...");

    // Read a module "ietf-inet-types".
    let mut ms = Modules::new();
    let data = yang.read(&ms, "ietf-inet-types").unwrap();

    match yang_parse(&data) {
        Ok((_, module)) => {
            println!("Module name: {}", module.name);
            println!("Module namespace: {}", module.namespace);
            println!("Module prefix: {}", module.prefix);
            ms.modules.insert(module.prefix.clone(), module);

            let entry = ms.modules.get(&"inet".to_string());
            if let Some(e) = entry {
                println!("XXX found {:?}", e);
            } else {
                println!("XXX not found")
            }
        }
        Err(e) => {
            println!("module parse: {:?}", e);
        }
    }

    // let ytype = YangType::new(TypeKind::Yenum);
    // println!("{:?}", ytype);

    let literal = r#"'collection abc' + 'hogehoge'"#;
    let result = quoted_string_list(literal);
    println!("{:?}", literal);
    println!("{:?}", result);

    // let literal = "\"urn:ietf:params:xml:ns:yang:ietf-inet-types\"";
    // println!("{}", literal);

    // let literal = r"\na";
    // let result = escape_code(literal);
    // println!("{:?}", result);

    // let literal = r#"main-routine_1 "#;
    // let result = nonescaped_string(literal);
    // println!("{:?}", result);

    // let literal = r#""hoge\thoga\nhoge""#;
    // println!("l: {:?}", literal);
    // match double_quoted_string(literal) {
    //     Ok((_, o)) => {
    //         println!("output: {}", o);
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // }
}
