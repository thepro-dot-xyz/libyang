use libyang::Yang;

fn main() {
    println!("hello");
    let mut yang = Yang::new();
    yang.add_path("/home/kunihiro/openconfigd/yang:/home/kunihiro/bin");
    println!("{:?}", yang.paths());
}
