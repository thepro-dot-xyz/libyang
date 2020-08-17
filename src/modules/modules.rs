use std::collections::HashMap;

pub struct Modules {
    pub modules: HashMap<String, Module>,
}

// RFC7950 7.1.1.  The module's Substatements
//
// +--------------+---------+-------------+
// | substatement | section | cardinality |
// +--------------+---------+-------------+
// | anydata      | 7.10    | 0..n        |
// | anyxml       | 7.11    | 0..n        |
// | augment      | 7.17    | 0..n        |
// | choice       | 7.9     | 0..n        |
// | contact      | 7.1.8   | 0..1        |
// | container    | 7.5     | 0..n        |
// | description  | 7.21.3  | 0..1        |
// | deviation    | 7.20.3  | 0..n        |
// | extension    | 7.19    | 0..n        |
// | feature      | 7.20.1  | 0..n        |
// | grouping     | 7.12    | 0..n        |
// | identity     | 7.18    | 0..n        |
// | import       | 7.1.5   | 0..n        |
// | include      | 7.1.6   | 0..n        |
// | leaf         | 7.6     | 0..n        |
// | leaf-list    | 7.7     | 0..n        |
// | list         | 7.8     | 0..n        |
// | namespace    | 7.1.3   | 1           |
// | notification | 7.16    | 0..n        |
// | organization | 7.1.7   | 0..1        |
// | prefix       | 7.1.4   | 1           |
// | reference    | 7.21.4  | 0..1        |
// | revision     | 7.1.9   | 0..n        |
// | rpc          | 7.14    | 0..n        |
// | typedef      | 7.3     | 0..n        |
// | uses         | 7.13    | 0..n        |
// | yang-version | 7.1.2   | 1           |
// +--------------+---------+-------------+
//
// In RFC6020 (YANG 1), yang-version cardinality is 0..1.

#[derive(Default, Debug)]
pub struct Module {
    pub name: String,
    pub namespace: String,
    pub prefix: String,
    pub organization: Option<String>,
    pub contact: Option<String>,
    pub description: Option<String>,
    pub revisions: Vec<RevisionNode>,
}

impl Modules {
    pub fn new() -> Self {
        Modules {
            modules: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct NamespaceNode {
    pub name: String,
}

impl NamespaceNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct PrefixNode {
    pub name: String,
}

impl PrefixNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OrganizationNode {
    pub name: String,
}

impl OrganizationNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ContactNode {
    pub name: String,
}

impl ContactNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DescriptionNode {
    pub name: String,
}

impl DescriptionNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct ReferenceNode {
    pub name: String,
}

impl ReferenceNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct RevisionNode {
    pub name: String,
    pub description: Option<String>,
    pub reference: Option<String>,
}
