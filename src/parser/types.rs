use crate::modules::*;
use crate::parser::*;
use crate::Node;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, digit0, multispace0, multispace1, one_of};
use nom::combinator::{map, recognize};
use nom::multi::{many0, separated_nonempty_list};
use nom::sequence::{pair, tuple};
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

fn is_digit_value(c: char) -> bool {
    c.is_digit(10)
}

fn digit_parse(s: &str) -> IResult<&str, &str> {
    take_while1(is_digit_value)(s)
}

pub fn value_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("value")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = alt((double_quoted_string, digit_parse))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
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

fn int_sub_parse(s: &str) -> IResult<&str, Vec<Node>> {
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

fn type_intx_parse(s: &str, type_string: String, type_kind: TypeKind) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag(type_string.as_str())(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((int_sub_parse, semicolon_end_parse))(s)?;

    let node = TypeNode::new(type_kind);
    Ok((s, Node::Type(Box::new(node))))
}

fn type_int8_parse(s: &str) -> IResult<&str, Node> {
    type_intx_parse(s, String::from("int8"), TypeKind::Yint8)
}

fn type_int16_parse(s: &str) -> IResult<&str, Node> {
    type_intx_parse(s, String::from("int16"), TypeKind::Yint16)
}

fn type_int32_parse(s: &str) -> IResult<&str, Node> {
    type_intx_parse(s, String::from("int32"), TypeKind::Yint32)
}

fn type_int64_parse(s: &str) -> IResult<&str, Node> {
    type_intx_parse(s, String::from("int64"), TypeKind::Yint64)
}

fn type_uint8_parse(s: &str) -> IResult<&str, Node> {
    type_intx_parse(s, String::from("uint8"), TypeKind::Yuint8)
}

fn type_uint16_parse(s: &str) -> IResult<&str, Node> {
    type_intx_parse(s, String::from("uint16"), TypeKind::Yuint16)
}

fn type_uint32_parse(s: &str) -> IResult<&str, Node> {
    type_intx_parse(s, String::from("uint32"), TypeKind::Yuint32)
}

fn type_uint64_parse(s: &str) -> IResult<&str, Node> {
    type_intx_parse(s, String::from("uint64"), TypeKind::Yuint64)
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

fn type_boolean_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("boolean")(s)?;
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

// Single statement 'keyword: identity;'
fn single_identity_parse(s: &str, key: String) -> IResult<&str, &str> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(key.as_str())(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = alt((path_identifier, identifier))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, v))
}

pub fn base_parse(s: &str) -> IResult<&str, Node> {
    let (s, v) = single_identity_parse(s, String::from("base"))?;
    let node = BaseNode::new(v.to_owned());
    Ok((s, Node::Base(Box::new(node))))
}

fn type_identityref_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("identityref")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, enums) = many0(base_parse)(s)?;
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
        default_parse,
        description_parse,
        reference_parse,
        types_parse,
        status_parse,
    )))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;

    let node = TypedefNode::new(String::from(ident), find_type_node(&mut nodes));
    Ok((s, Node::Typedef(Box::new(node))))
}

// We owe integer parsing logic from https://codeandbitters.com/lets-build-a-parser/.
//
// When type is unsinged integer value is parsed into u64 otherwise parsed into
// i64.

fn digit1to9(input: &str) -> IResult<&str, char> {
    one_of("123456789")(input)
}

fn uint_parse(input: &str) -> IResult<&str, &str> {
    alt((tag("0"), recognize(pair(digit1to9, digit0))))(input)
}

fn negative_parse(input: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("-"), digit1to9, digit0)))(input)
}

fn int_parse(input: &str) -> IResult<&str, &str> {
    alt((negative_parse, uint_parse))(input)
}

pub fn uint_parse_value(input: &'static str, mmax: &'static str) -> IResult<&'static str, UintVal> {
    let parser = alt((tag(mmax), uint_parse));
    map(parser, |s| match s {
        "max" => UintVal::Max,
        "min" => UintVal::Min,
        x => {
            let n = x.parse::<u64>().unwrap();
            UintVal::Val(n)
        }
    })(input)
}

pub fn int_parse_value(input: &'static str, mmax: &'static str) -> IResult<&'static str, IntVal> {
    let parser = alt((tag(mmax), int_parse));
    map(parser, |s| match s {
        "max" => IntVal::Max,
        "min" => IntVal::Min,
        x => {
            let n = x.parse::<i64>().unwrap();
            IntVal::Val(n)
        }
    })(input)
}

pub fn range_int_single_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((tag("min"), tag("max"), int_parse))(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, Node::EmptyNode))
}

pub fn range_uint_single_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((tag("min"), tag("max"), uint_parse))(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, Node::EmptyNode))
}

pub fn range_int_parse(s: &'static str) -> IResult<&'static str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, v1) = int_parse_value(s, "min")?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("..")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, v2) = int_parse_value(s, "max")?;
    println!("v1 {:?}", v1);
    println!("v2 {:?}", v2);
    Ok((s, Node::EmptyNode))
}

pub fn range_uint_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, v1) = alt((tag("min"), uint_parse))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("..")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, v2) = alt((tag("max"), uint_parse))(s)?;
    println!("v1 {}", v1);
    println!("v2 {}", v2);
    Ok((s, Node::EmptyNode))
}

pub fn range_uint_multi_parse(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, v) = separated_nonempty_list(
        permutation((multispace0, char('|'), multispace0)),
        alt((range_uint_parse, range_uint_single_parse)),
    )(s)?;
    Ok((s, v))
}

pub fn types_parse(s: &str) -> IResult<&str, Node> {
    alt((
        type_int8_parse,
        type_int16_parse,
        type_int32_parse,
        type_int64_parse,
        type_uint8_parse,
        type_uint16_parse,
        type_uint32_parse,
        type_uint64_parse,
        type_string_parse,
        type_boolean_parse,
        type_enumeration_parse,
        type_union_parse,
        type_identifier_parse,
        type_path_identifier_parse,
        type_identityref_parse,
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err::Error;

    #[test]
    fn yang_type() {
        // let ytype = YangType::new(TypeKind::Yenum);
        // println!("{:?}", ytype);
    }

    #[test]
    fn test_value_parse() {
        let literal = "1a";
        let result = value_parse(literal);
        println!("XXX test_value_parse: {:?}", result);
        //assert_eq!(result, Ok(("", true)));
    }

    #[test]
    fn test_base_parse() {
        let literal = "base if:interface-type;";
        let result = base_parse(literal);
        println!("XXX test_base_parse: {:?}", result);
        //assert_eq!(result, Ok(("", true)));
    }

    #[test]
    fn test_identityref_parse() {
        let literal = r#"
        type identityref {
            base interface-type;
        }"#;
        let result = type_identityref_parse(literal);
        println!("XXX test_identityref_parse: {:?}", result);
    }

    // "range" has digit, "min", "max" as <value> statement. Range can be
    // specified <value>..<value> or just simple <value>. We can have
    // multiple range with separating pipe
    // <value>..<value>|<value>..<value>. So "range" can have multiple set
    // of range. When type is inherited, range must be more specific than
    // parent type range. Following example from RFC7951 shows illegal range
    // specification when it inherit range from parent.

    // 9.2.5.  Usage Example
    //
    // typedef my-base-int32-type {
    //     type int32 {
    //         range "1..4 | 10..20";
    //     }
    // }
    //
    // typedef my-type1 {
    //     type my-base-int32-type {
    //         // legal range restriction
    //         range "11..max"; // 11..20
    //     }
    // }
    //
    // typedef my-type2 {
    //     type my-base-int32-type {
    //         // illegal range restriction
    //         range "11..100";
    //     }
    // }
    #[test]
    fn test_uint_parse() {
        struct Test {
            input: &'static str,
            output: IResult<&'static str, &'static str>,
        };
        let tests = [
            Test {
                input: "0",
                output: Ok(("", "0")),
            },
            Test {
                input: "00",
                output: Ok(("0", "0")),
            },
            Test {
                input: "0123",
                output: Ok(("123", "0")),
            },
            Test {
                input: "123",
                output: Ok(("", "123")),
            },
            Test {
                input: "2020",
                output: Ok(("", "2020")),
            },
            Test {
                input: "-2020",
                output: Err(Error(("-2020", ErrorKind::OneOf))),
            },
        ];
        for t in &tests {
            let result = uint_parse(t.input);
            assert_eq!(result, t.output);
        }
    }

    #[test]
    fn test_negative_parse() {
        struct Test {
            input: &'static str,
            output: IResult<&'static str, &'static str>,
        };
        let tests = [
            Test {
                input: "0",
                output: Err(Error(("0", ErrorKind::Tag))),
            },
            Test {
                input: "-0",
                output: Err(Error(("0", ErrorKind::OneOf))),
            },
            Test {
                input: "-1",
                output: Ok(("", "-1")),
            },
            Test {
                input: "-123",
                output: Ok(("", "-123")),
            },
            Test {
                input: "-1020",
                output: Ok(("", "-1020")),
            },
            Test {
                input: "2020",
                output: Err(Error(("2020", ErrorKind::Tag))),
            },
        ];
        for t in &tests {
            let result = negative_parse(t.input);
            assert_eq!(result, t.output);
        }
    }

    #[test]
    fn test_int_parse() {
        struct Test {
            input: &'static str,
            output: IResult<&'static str, &'static str>,
        };
        let tests = [
            Test {
                input: "0",
                output: Ok(("", "0")),
            },
            Test {
                input: "-0",
                output: Err(Error(("-0", ErrorKind::OneOf))),
            },
            Test {
                input: "1",
                output: Ok(("", "1")),
            },
            Test {
                input: "-1",
                output: Ok(("", "-1")),
            },
            Test {
                input: "123",
                output: Ok(("", "123")),
            },
            Test {
                input: "-123",
                output: Ok(("", "-123")),
            },
            Test {
                input: "-1020",
                output: Ok(("", "-1020")),
            },
            Test {
                input: "2020",
                output: Ok(("", "2020")),
            },
        ];
        for t in &tests {
            let result = int_parse(t.input);
            assert_eq!(result, t.output);
        }
    }

    #[test]
    fn test_range_uint_parse() {
        let literal = "1 .. 20";
        let result = range_uint_parse(literal);
        println!("{:?}", result);

        let literal = "0..20";
        let result = range_uint_parse(literal);
        println!("{:?}", result);

        let literal = "-1..20";
        let result = range_uint_parse(literal);
        println!("{:?}", result);

        let literal = "min..20";
        let result = range_uint_parse(literal);
        println!("{:?}", result);

        let literal = "max..20";
        let result = range_uint_parse(literal);
        println!("{:?}", result);

        let literal = "min..max";
        let result = range_uint_parse(literal);
        println!("{:?}", result);
    }

    #[test]
    fn test_rainge_uint_multi_parse() {
        let literal = "0..1";
        let result = range_uint_multi_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "1..20 | 22..24";
        let result = range_uint_multi_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "1..20 | 22..24 | 35..100";
        let result = range_uint_multi_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "0 | 1..10";
        let result = range_uint_multi_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);
    }

    #[test]
    fn test_uint_single_parse() {
        let literal = "128";
        let result = range_uint_single_parse(literal);
        println!("XXX range_uint_single_parse: {:?}", result);

        let literal = "max";
        let result = range_uint_single_parse(literal);
        println!("XXX range_uint_single_parse: {:?}", result);
    }

    #[test]
    fn test_int_parse_value() {
        let literal = "-128";
        let result = int_parse_value(literal, "min");
        println!("XXX test_int_parse_value: {:?}", result);

        let literal = "max";
        let result = int_parse_value(literal, "max");
        println!("XXX test_int_parse_value: {:?}", result);
    }

    // let literal = "0..1";
    // range "0 | 30..65535";
    // range "1..14 | 36 | 40 | 44| 48 | 52 | 56 | 60 | 64 | 100 | 104 | 108 | 112 | 116 | 120 | 124 | 128 | 132 | 136 | 140 | 144 | 149 | 153 | 157 | 161 | 165";
    // range "68..max";
}
