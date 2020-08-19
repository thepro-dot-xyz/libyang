use libyang::*;

#[test]
fn parse_ietf_inet_types_test() {
    let mut yang = Yang::new();
    yang.add_path("/etc/openconfigd/yang:yang/...");

    // Read a module.
    let mut ms = Modules::new();
    let yang_name = "ietf-inet-types";
    let data = yang.read(&ms, yang_name).unwrap();

    match yang_parse(&data) {
        Ok((_, module)) => {
            ms.modules.insert(module.prefix.to_owned(), module);

            let entry = ms.modules.get(&"inet".to_string());
            if let Some(_) = entry {
                // Success.
            } else {
                // Module not found.
                panic!("modules can't find")
            }
        }
        Err(e) => {
            panic!("module parse error: {:?}", e);
        }
    }
}

#[test]
fn parse_ietf_yang_types_test() {
    let mut yang = Yang::new();
    yang.add_path("/etc/openconfigd/yang:yang/...");

    // Read a module.
    let mut ms = Modules::new();
    let yang_name = "ietf-yang-types";
    let data = yang.read(&ms, yang_name).unwrap();

    match yang_parse(&data) {
        Ok((_, module)) => {
            ms.modules.insert(module.prefix.to_owned(), module);

            let entry = ms.modules.get(&"yang".to_string());
            if let Some(_) = entry {
                // Success.
            } else {
                // Module not found.
                panic!("modules can't find")
            }
        }
        Err(e) => {
            panic!("module parse error: {:?}", e);
        }
    }
}
