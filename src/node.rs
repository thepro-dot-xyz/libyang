use crate::modules::*;
use crate::types::*;

#[derive(Debug, PartialEq)]
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
    Revision(Box<RevisionNode>),
    Typedef(Box<TypedefNode>),
    Default(Box<DefaultNode>),
    YangVersion(Box<YangVersionNode>),
    Import(Box<ImportNode>),
    Include(Box<IncludeNode>),
    RevisionDate(Box<RevisionDateNode>),
    Identity(Box<IdentityNode>),
    Feature(Box<FeatureNode>),
}
