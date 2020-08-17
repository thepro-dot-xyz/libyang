use super::*;
use crate::modules::*;
use crate::Node;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_until, take_while, take_while1};
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

pub fn double_quoted_string(s: &str) -> IResult<&str, &str> {
    // let parser = delimited(tag("\""), string_body, tag("\""));
    // map_res(parser, |x| unescape(x))(s)
    delimited(tag("\""), string_body, tag("\""))(s)
}

pub fn quoted_string(s: &str) -> IResult<&str, &str> {
    let (s, _) = delimited(tag("'"), many0(none_of("'")), tag("'"))(s)?;
    Ok((s, ""))
}

pub fn quoted_string_list(s: &str) -> IResult<&str, &str> {
    let (s, _) = separated_list(
        permutation((multispace0, char('+'), multispace0)),
        quoted_string,
    )(s)?;
    Ok((s, ""))
}

pub fn c_comment_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("/*")(s)?;
    let (s, _) = take_until("*/")(s)?;
    let (s, _) = tag("*/")(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, Node::EmptyNode))
}

// Single statement 'keyword: "double quoted string";'
pub fn single_statement_parse(s: &str, key: String) -> IResult<&str, &str> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(key.as_str())(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = double_quoted_string(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, v))
}

pub fn description_parse(s: &str) -> IResult<&str, Node> {
    let (s, v) = single_statement_parse(s, String::from("description"))?;
    let n = DescriptionNode::new(v.to_owned());
    Ok((s, Node::Description(Box::new(n))))
}

pub fn reference_parse(s: &str) -> IResult<&str, Node> {
    let (s, v) = single_statement_parse(s, String::from("reference"))?;
    let node = ReferenceNode::new(v.to_owned());
    Ok((s, Node::Reference(Box::new(node))))
}

pub fn semicolon_end_parse(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, _) = tag(";")(s)?;
    Ok((s, vec![]))
}

fn module_parse(s: &str) -> IResult<&str, Node> {
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
    let node = match k {
        "namespace" => {
            let n = NamespaceNode::new(v.to_owned());
            Node::Namespace(Box::new(n))
        }
        "prefix" => {
            let n = PrefixNode::new(v.to_owned());
            Node::Prefix(Box::new(n))
        }
        "organization" => {
            let n = OrganizationNode::new(v.to_owned());
            Node::Organization(Box::new(n))
        }
        "contact" => {
            let n = ContactNode::new(v.to_owned());
            Node::Contact(Box::new(n))
        }
        "description" => {
            let n = DescriptionNode::new(v.to_owned());
            Node::Description(Box::new(n))
        }
        _ => Node::EmptyNode,
    };
    Ok((s, node))
}

pub fn yang_parse(s: &str) -> IResult<&str, Module> {
    let (s, _) = tag("module")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, ident) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, nodes) = many0(alt((
        module_parse,
        revision_parse,
        c_comment_parse,
        typedef_parse,
    )))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;

    let mut module = Module::default();
    module.name = String::from(ident);
    for node in &nodes {
        match node {
            Node::Namespace(n) => {
                module.namespace = n.name.to_owned();
            }
            Node::Prefix(n) => {
                module.prefix = n.name.to_owned();
            }
            Node::Organization(n) => {
                module.organization = Some(n.name.to_owned());
            }
            Node::Contact(n) => {
                module.contact = Some(n.name.to_owned());
            }
            Node::Description(n) => {
                module.description = Some(n.name.to_owned());
            }
            _ => {}
        }
    }
    Ok((s, module))
}
