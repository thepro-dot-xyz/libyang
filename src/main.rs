use libyang::{Modules, Yang};

fn main() {
    // Allocate a new Yang.
    let mut yang = Yang::new();
    yang.add_path("/home/kunihiro/openconfigd/yang:/home/kunihiro/...");
    println!("{:?}", yang.paths());

    // Read a module "ietf-dhcp".
    let ms = Modules::new();
    yang.read(&ms, "ietf-dhcp").unwrap();
}
