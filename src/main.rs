use libyang::{Modules, Yang};

fn main() {
    // Allocate a new Yang.
    let mut yang = Yang::new();
    yang.add_path("/etc/openconfigd/yang:tests/...");
    println!("{:?}", yang.paths());

    // Read a module "ietf-dhcp".
    let ms = Modules::new();
    yang.read(&ms, "ietf-inet-types").unwrap();
}
