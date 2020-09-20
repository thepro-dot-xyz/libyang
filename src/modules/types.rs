use crate::Node;

#[derive(Debug, PartialEq)]
pub enum TypeKind {
    Ynone,
    Yint8,
    Yint16,
    Yint32,
    Yint64,
    Yuint8,
    Yuint16,
    Yuint32,
    Yuint64,
    // Yenum,
}

#[derive(Debug, PartialEq)]
pub struct TypeNode {
    pub kind: TypeKind,
    pub name: String,
}

impl Default for TypeNode {
    fn default() -> Self {
        Self {
            name: String::from(""),
            kind: TypeKind::Ynone,
        }
    }
}

impl TypeNode {
    pub fn new(kind: TypeKind) -> Self {
        TypeNode {
            kind: kind,
            ..Default::default()
        }
    }

    pub fn match_with(_str: &str) {}
}

#[derive(Debug, Default, PartialEq)]
pub struct TypedefNode {
    pub name: String,
    pub typ: Option<Node>,
}

impl TypedefNode {
    pub fn new(name: String, typ: Option<Node>) -> Self {
        TypedefNode {
            name: name,
            typ: typ,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ValueNode {
    pub name: String,
    pub nodes: (),
}

#[derive(Debug)]
pub struct Uint8Node {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct EnumNode {
    pub name: String,
    pub nodes: (Vec<Node>,),
}

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default, PartialEq, new)]
pub struct DefaultNode {
    pub name: String,
}

#[derive(Debug, PartialEq, new)]
pub struct BaseNode {
    pub name: String,
}