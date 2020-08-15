use libyang::{Module, Modules, Yang};

// use escape8259::unescape;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_until, take_while, take_while1, take_while_m_n};
use nom::character::complete::{anychar, char, multispace0, multispace1, none_of};
use nom::combinator::{recognize, verify};
use nom::multi::{many0, separated_list};
use nom::sequence::{delimited, pair};
use nom::IResult;

// RFC7950 6.2.  Identifiers
//     Identifiers are used to identify different kinds of YANG items by
//     name.  Each identifier starts with an uppercase or lowercase ASCII
//     letter or an underscore character, followed by zero or more ASCII
//     letters, digits, underscore characters, hyphens, and dots.
//     Implementations MUST support identifiers up to 64 characters in
//     length and MAY support longer identifiers.  Identifiers are case
//     sensitive.  The identifier syntax is formally defined by the rule
//     "identifier" in Section 14.  Identifiers can be specified as quoted
//     or unquoted strings.

pub fn is_identifier(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.'
}

pub fn identifier(s: &str) -> IResult<&str, &str> {
    recognize(pair(
        verify(anychar, |c: &char| c.is_ascii_alphabetic() || c == &'_'),
        take_while(is_identifier),
    ))(s)
}

pub fn path_identifier(s: &str) -> IResult<&str, &str> {
    let (s, _) = identifier(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, id) = identifier(s)?;
    Ok((s, id))
}

// RFC7950 6.1.3.  Quoting
//     Within a double-quoted string (enclosed within " "), a backslash
//     character introduces a representation of a special character, which
//     depends on the character that immediately follows the backslash:
//
//     \n      newline
//     \t      a tab character
//     \"      a double quote
//     \\      a single backslash
//
//     The backslash MUST NOT be followed by any other character.

fn is_nonescaped_string_char(c: char) -> bool {
    let cv = c as u32;
    // 0x22 is double quote and 0x5C is backslash.
    (cv == 0x0a) || (cv == 0x0d) || ((cv >= 0x20) && (cv != 0x22) && (cv != 0x5c))
}

fn nonescaped_string(s: &str) -> IResult<&str, &str> {
    take_while1(is_nonescaped_string_char)(s)
}

fn escape_code(s: &str) -> IResult<&str, &str> {
    recognize(pair(
        tag("\\"),
        alt((tag("n"), tag("t"), tag("\""), tag("\\"))),
    ))(s)
}

fn string_body(s: &str) -> IResult<&str, &str> {
    recognize(many0(alt((nonescaped_string, escape_code))))(s)
}

fn double_quoted_string(s: &str) -> IResult<&str, &str> {
    // let parser = delimited(tag("\""), string_body, tag("\""));
    // map_res(parser, |x| unescape(x))(s)
    delimited(tag("\""), string_body, tag("\""))(s)
}

fn quoted_string(s: &str) -> IResult<&str, &str> {
    let (s, _) = delimited(tag("'"), many0(none_of("'")), tag("'"))(s)?;
    Ok((s, ""))
}

fn quoted_string_list(s: &str) -> IResult<&str, &str> {
    let (s, _) = separated_list(
        permutation((multispace0, char('+'), multispace0)),
        quoted_string,
    )(s)?;
    Ok((s, ""))
}

fn revision_date_parse(s: &str) -> IResult<&str, &str> {
    // YYYY-MM-DD format.
    let (s, year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(s)?;
    let (s, _) = char('-')(s)?;
    let (s, month) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(s)?;
    let (s, _) = char('-')(s)?;
    let (s, day) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(s)?;
    println!("revision parsed: {}-{}-{}", year, month, day);
    Ok((s, ""))
}

fn revision_date_quoted_parse(s: &str) -> IResult<&str, &str> {
    // Quoted "YYYY-MM-DD" format.
    let (s, _) = char('"')(s)?;
    let (s, o) = revision_date_parse(s)?;
    let (s, _) = char('"')(s)?;
    Ok((s, o))
}

fn revision_date_token_parse(s: &str) -> IResult<&str, &str> {
    alt((revision_date_parse, revision_date_quoted_parse))(s)
}

fn revision_sub_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, k) = alt((tag("description"), tag("reference")))(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = double_quoted_string(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, (k, v)))
}

// Module:top
fn revision_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, k) = tag("revision")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = revision_date_token_parse(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, _) = many0(revision_sub_parse)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    Ok((s, (k, v)))
}

// Module:top
fn module_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, k) = alt((
        tag("namespace"),
        tag("prefix"),
        tag("organization"),
        tag("contact"),
        tag("description"),
    ))(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = double_quoted_string(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, (k, v)))
}

// Module:top
fn c_comment_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("/*")(s)?;
    let (s, _) = take_until("*/")(s)?;
    let (s, _) = tag("*/")(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, ("", "")))
}

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

#[derive(Debug)]
enum TypeKind {
    Ynone,
    // Yint8,
    Yenum,
}

#[derive(Debug)]
struct YangType {
    name: String,
    kind: TypeKind,
}

impl Default for YangType {
    fn default() -> Self {
        Self {
            name: String::from(""),
            kind: TypeKind::Ynone,
        }
    }
}

impl YangType {
    fn new(kind: TypeKind) -> Self {
        YangType {
            kind: kind,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub enum AllNode {
    EmptyNode,
    ValueNode(Box<ValueNode>),
    DescriptionNode(Box<DescriptionNode>),
    EnumNode(Box<EnumNode>),
    EnumerationNode(Box<EnumerationNode>),
}

#[derive(Debug)]
pub struct ValueNode {
    pub name: String,
    pub nodes: (),
}

#[derive(Debug)]
pub struct DescriptionNode {
    pub name: String,
    pub nodes: (),
}

#[derive(Debug)]
pub struct EnumNode {
    pub name: String,
    pub nodes: (Vec<AllNode>,),
}

#[derive(Debug, Default)]
pub struct EnumerationNode {
    pub name: String,
    pub nodes: (Vec<AllNode>,),
    pub min: i32,
    pub max: i32,
}

impl EnumerationNode {
    fn new(nodes: Vec<AllNode>) -> Self {
        EnumerationNode {
            name: String::from(""),
            nodes: (nodes,),
            min: 0,
            max: 0,
        }
    }
}

#[derive(Debug)]
pub struct TypedefNode {
    pub name: String,
    pub typ: Option<AllNode>,
}

impl TypedefNode {
    fn new(name: String, typ: Option<AllNode>) -> Self {
        TypedefNode {
            name: name,
            typ: typ,
        }
    }
}

// Single statement 'keyword: "double quoted string";'
fn single_statement_parse(s: &str, key: String) -> IResult<&str, &str> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(key.as_str())(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = double_quoted_string(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, v))
}

fn description_parse(s: &str) -> IResult<&str, AllNode> {
    let (s, v) = single_statement_parse(s, String::from("description"))?;
    let node = DescriptionNode {
        name: String::from(v),
        nodes: (),
    };
    Ok((s, AllNode::DescriptionNode(Box::new(node))))
}

fn value_parse(s: &str) -> IResult<&str, AllNode> {
    let (s, v) = single_statement_parse(s, String::from("value"))?;
    let node = ValueNode {
        name: String::from(v),
        nodes: (),
    };
    Ok((s, AllNode::ValueNode(Box::new(node))))
}

fn reference_parse(s: &str) -> IResult<&str, AllNode> {
    let (s, v) = single_statement_parse(s, String::from("reference"))?;
    let node = DescriptionNode {
        name: String::from(v),
        nodes: (),
    };
    Ok((s, AllNode::DescriptionNode(Box::new(node))))
}

fn enum_sub_parse(s: &str) -> IResult<&str, Vec<AllNode>> {
    let (s, _) = char('{')(s)?;
    let (s, nodes) = many0(alt((description_parse, value_parse)))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    Ok((s, nodes))
}

fn semicolon_end_parse(s: &str) -> IResult<&str, Vec<AllNode>> {
    let (s, _) = tag(";")(s)?;
    Ok((s, vec![]))
}

fn enum_parse(s: &str) -> IResult<&str, AllNode> {
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
    Ok((s, AllNode::EnumNode(Box::new(node))))
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

fn uint_sub_parse(s: &str) -> IResult<&str, Vec<AllNode>> {
    let (s, _) = char('{')(s)?;
    let (s, _) = many0(range_parse)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    Ok((s, vec![]))
}

fn type_uint8_parse(s: &str) -> IResult<&str, AllNode> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("uint8")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((uint_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, AllNode::EmptyNode))
}

fn type_uint16_parse(s: &str) -> IResult<&str, AllNode> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("uint16")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((uint_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, AllNode::EmptyNode))
}

fn type_uint32_parse(s: &str) -> IResult<&str, AllNode> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = tag("uint32")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((uint_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, AllNode::EmptyNode))
}

fn type_enumeration_parse(s: &str) -> IResult<&str, AllNode> {
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

    Ok((s, AllNode::EnumerationNode(Box::new(node))))
}

fn type_parse(s: &str) -> IResult<&str, AllNode> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("type")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = path_identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(";")(s)?;
    Ok((s, AllNode::EmptyNode))
}

fn type_union_parse(s: &str) -> IResult<&str, AllNode> {
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
    Ok((s, AllNode::EnumerationNode(Box::new(node))))
}

pub fn find_type_node(nodes: &mut Vec<AllNode>) -> Option<AllNode> {
    let index = nodes.iter().position(|x| match x {
        AllNode::EnumerationNode(_) => true,
        _ => false,
    })?;
    Some(nodes.swap_remove(index))
}

// Module:top
fn typedef_parse(s: &str) -> IResult<&str, (&str, &str)> {
    let (s, _) = multispace0(s)?;
    let (s, k) = tag("typedef")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, ident) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, mut nodes) = many0(alt((
        type_uint8_parse,
        type_uint16_parse,
        type_uint32_parse,
        type_enumeration_parse,
        type_union_parse,
        description_parse,
        reference_parse,
    )))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;

    let node = TypedefNode::new(String::from(ident), find_type_node(&mut nodes));
    println!("{:?}", node);

    Ok((s, (k, ident)))
}

fn yang_parse(s: &str) -> IResult<&str, &str> {
    let (s, _) = tag("module")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, ident) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, vec) = many0(alt((
        module_parse,
        revision_parse,
        c_comment_parse,
        typedef_parse,
    )))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;

    let mut module = Module::default();
    module.name = String::from(ident);
    for (k, v) in vec {
        match k {
            "namespace" => {
                module.namespace = String::from(v);
            }
            "prefix" => {
                module.prefix = String::from(v);
            }
            "organization" => {
                module.organization = String::from(v);
            }
            "contact" => {
                module.contact = String::from(v);
            }
            "description" => {
                module.description = String::from(v);
            }
            _ => {}
        }
    }
    println!("{:?}", module);
    println!("{}", module.description);
    Ok((s, ident))
}

fn main() {
    // Allocate a new Yang.
    let mut yang = Yang::new();
    yang.add_path("/etc/openconfigd/yang:tests/...");
    // println!("{:?}", yang.paths());

    // Read a module "ietf-dhcp".
    let ms = Modules::new();
    let data = yang.read(&ms, "ietf-inet-types").unwrap();
    // println!("{}", data);

    match yang_parse(&data) {
        Ok((_, o)) => {
            println!("Module {:?} parse success", o);
        }
        Err(e) => {
            println!("module parse: {:?}", e);
        }
    }

    let ytype = YangType::new(TypeKind::Yenum);
    println!("{:?}", ytype);

    let literal = r#"'collection abc' + 'hogehoge'"#;
    let result = quoted_string_list(literal);
    println!("{:?}", literal);
    println!("{:?}", result);

    // Move to test.
    // let revision = "2020-08-10";
    // println!("{:?}", revision_date_parse(revision));
    // let revision_q = "\"2020-08-10\"";
    // println!("{:?}", revision_date_quoted_parse(revision_q));

    // println!("{:?}", revision_date_token_parse(revision));
    // println!("{:?}", revision_date_token_parse(revision_q));

    // let literal = "\"urn:ietf:params:xml:ns:yang:ietf-inet-types\"";
    // println!("{}", literal);

    // let literal = r"\na";
    // let result = escape_code(literal);
    // println!("{:?}", result);

    // let literal = r#"main-routine_1 "#;
    // let result = nonescaped_string(literal);
    // println!("{:?}", result);

    // let literal = r#""hoge\thoga\nhoge""#;
    // println!("l: {:?}", literal);
    // match double_quoted_string(literal) {
    //     Ok((_, o)) => {
    //         println!("output: {}", o);
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // }
}
