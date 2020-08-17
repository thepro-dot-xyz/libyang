use crate::modules::*;
use crate::types::*;

#[derive(Debug)]
pub enum Node {
    EmptyNode,
    Namespace(Box<NamespaceNode>),
    Prefix(Box<PrefixNode>),
    Organization(Box<OrganizationNode>),
    Contact(Box<ContactNode>),
    Description(Box<DescriptionNode>),
    Reference(Box<ReferenceNode>),
    ValueNode(Box<ValueNode>),
    EnumNode(Box<EnumNode>),
    EnumerationNode(Box<EnumerationNode>),
}
