# libyang

`libyang` is a parser library for YANG (RFC6020, RFC7950) written in Rust
programming language.

## Example

``` rust
fn main() {
    // Allocate a new Yang.
    let mut yang = Yang::new();
    yang.add_path("/etc/openconfigd/yang:yang/...");

    // Read a module.
    let mut ms = Modules::new();
    let yang_name = "iana-if-type";
    let data = yang.read(&ms, yang_name).unwrap();

    match yang_parse(&data) {
        Ok((_, module)) => {
            ms.modules.insert(module.prefix.to_owned(), module);

            let entry = ms.modules.get(&"ianaift".to_string());
            if let Some(e) = entry {
                println!("Module dump: {:?}", e);
            } else {
                println!("Module not found")
            }
        }
        Err(e) => {
            println!("Module parse error: {:?}", e);
        }
    }
}

```

## License

Rocket is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT License ([LICENSE-MIT](LICENSE-MIT))
