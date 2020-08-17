use crate::Node;

#[derive(Debug)]
pub struct ValueNode {
    pub name: String,
    pub nodes: (),
}

#[derive(Debug)]
pub struct Uint8Node {
    pub name: String,
}

#[derive(Debug)]
pub struct EnumNode {
    pub name: String,
    pub nodes: (Vec<Node>,),
}

#[derive(Debug, Default)]
pub struct EnumerationNode {
    pub name: String,
    pub nodes: (Vec<Node>,),
    pub min: i32,
    pub max: i32,
}

impl EnumerationNode {
    pub fn new(nodes: Vec<Node>) -> Self {
        EnumerationNode {
            name: String::from(""),
            nodes: (nodes,),
            min: 0,
            max: 0,
        }
    }
}
