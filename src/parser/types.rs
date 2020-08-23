use super::*;
use crate::types::*;
use crate::Node;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::multi::many0;
use nom::IResult;

// 4.2.4.  Built-In Types
//
//    YANG has a set of built-in types, similar to those of many
//    programming languages, but with some differences due to special
//    requirements of network management.  The following table summarizes
//    the built-in types discussed in Section 9:
//
//        +---------------------+-------------------------------------+
//        | Name                | Description                         |
//        +---------------------+-------------------------------------+
//        | binary              | Any binary data                     |
//        | bits                | A set of bits or flags              |
//        | boolean             | "true" or "false"                   |
//        | decimal64           | 64-bit signed decimal number        |
//        | empty               | A leaf that does not have any value |
//        | enumeration         | One of an enumerated set of strings |
//        | identityref         | A reference to an abstract identity |
//        | instance-identifier | A reference to a data tree node     |
//        | int8                | 8-bit signed integer                |
//        | int16               | 16-bit signed integer               |
//        | int32               | 32-bit signed integer               |
//        | int64               | 64-bit signed integer               |
//        | leafref             | A reference to a leaf instance      |
//        | string              | A character string                  |
//        | uint8               | 8-bit unsigned integer              |
//        | uint16              | 16-bit unsigned integer             |
//        | uint32              | 32-bit unsigned integer             |
//        | uint64              | 64-bit unsigned integer             |
//        | union               | Choice of member types              |
//        +---------------------+-------------------------------------+

// #[derive(Debug)]
// enum TypeKind {
//     Ynone,
//     // Yint8,
//     Yenum,
// }

// #[derive(Debug)]
// struct YangType {
//     name: String,
//     kind: TypeKind,
// }

// impl Default for YangType {
//     fn default() -> Self {
//         Self {
//             name: String::from(""),
//             kind: TypeKind::Ynone,
//         }
//     }
// }

// impl YangType {
//     fn new(kind: TypeKind) -> Self {
//         YangType {
//             kind: kind,
//             ..Default::default()
//         }
//     }
// }

// #[derive(Debug)]
// pub struct TypedefNode {
//     pub name: String,
//     pub typ: Option<Node>,
// }

// impl TypedefNode {
//     fn new(name: String, typ: Option<Node>) -> Self {
//         TypedefNode {
//             name: name,
//             typ: typ,
//         }
//     }
// }

fn value_parse(s: &str) -> IResult<&str, Node> {
    let (s, v) = single_statement_parse(s, String::from("value"))?;
    let node = ValueNode {
        name: String::from(v),
        nodes: (),
    };
    Ok((s, Node::ValueNode(Box::new(node))))
}

fn enum_sub_parse(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, _) = char('{')(s)?;
    let (s, nodes) = many0(alt((description_parse, value_parse)))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    Ok((s, nodes))
}

fn semicolon_end_parse(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, _) = tag(";")(s)?;
    Ok((s, vec![]))
}

fn enum_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("enum")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, ident) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, sub) = alt((enum_sub_parse, semicolon_end_parse))(s)?;
    let node = EnumNode {
        name: String::from(ident),
        nodes: (sub,),
    };
    Ok((s, Node::EnumNode(Box::new(node))))
}

fn range_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, k) = tag("range")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = double_quoted_string(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;

    Ok((s, (k, v)))
}

fn uint_sub_parse(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, _) = char('{')(s)?;
    let (s, _) = many0(range_parse)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    Ok((s, vec![]))
}

fn pattern_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, k) = tag("pattern")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _v) = quoted_string_list(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, (k, "")))
}

fn length_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, k) = tag("length")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = double_quoted_string(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, (k, v)))
}

fn path_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, k) = tag("path")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = double_quoted_string(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, (k, v)))
}

fn type_sub_parse(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, _) = char('{')(s)?;
    let (s, _) = many0(alt((pattern_parse, length_parse, path_parse)))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    Ok((s, vec![]))
}

fn type_uint8_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("uint8")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((uint_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, Node::EmptyNode))
}

fn type_uint16_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("uint16")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((uint_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, Node::EmptyNode))
}

fn type_uint32_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("uint32")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((uint_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, Node::EmptyNode))
}

fn type_uint64_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("uint64")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((uint_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, Node::EmptyNode))
}

fn type_string_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("string")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((type_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, Node::EmptyNode))
}

fn type_enumeration_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("enumeration")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, enums) = many0(enum_parse)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;

    let node = EnumerationNode::new(enums);

    Ok((s, Node::EnumerationNode(Box::new(node))))
}

fn type_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = path_identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(";")(s)?;
    Ok((s, Node::EmptyNode))
}

fn type_union_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("union")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, enums) = many0(type_parse)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    let node = EnumerationNode::new(enums);
    Ok((s, Node::EnumerationNode(Box::new(node))))
}

fn type_path_identifier_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = path_identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((type_sub_parse, semicolon_end_parse))(s)?;

    Ok((s, Node::EmptyNode))
}

fn type_identifier_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((type_sub_parse, semicolon_end_parse))(s)?;

    Ok((s, Node::EmptyNode))
}

pub fn find_type_node(nodes: &mut Vec<Node>) -> Option<Node> {
    let index = nodes.iter().position(|x| match x {
        Node::EnumerationNode(_) => true,
        _ => false,
    })?;
    Some(nodes.swap_remove(index))
}

pub fn default_parse(s: &str) -> IResult<&str, Node> {
    let (s, v) = single_statement_parse(s, String::from("default"))?;
    let n = DefaultNode::new(v.to_owned());
    Ok((s, Node::Default(Box::new(n))))
}

// Module:top
pub fn typedef_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _k) = tag("typedef")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, ident) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, mut nodes) = many0(alt((
        type_uint8_parse,
        type_uint16_parse,
        type_uint32_parse,
        type_uint64_parse,
        type_string_parse,
        type_enumeration_parse,
        type_union_parse,
        type_path_identifier_parse,
        type_identifier_parse,
        default_parse,
        description_parse,
        reference_parse,
    )))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;

    let node = TypedefNode::new(String::from(ident), find_type_node(&mut nodes));
    Ok((s, Node::Typedef(Box::new(node))))
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn yang_type() {
        // let ytype = YangType::new(TypeKind::Yenum);
        // println!("{:?}", ytype);
    }
}
