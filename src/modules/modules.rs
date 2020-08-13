pub struct Modules {}

#[derive(Default, Debug)]
pub struct Module {
    pub name: String,
    pub namespace: String,
    pub prefix: String,
    pub organization: String,
    pub contact: String,
    pub description: String,
}

impl Modules {
    pub fn new() -> Self {
        Modules {}
    }
}
