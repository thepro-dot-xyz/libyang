use crate::modules::*;
use crate::Node;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom::IResult;

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
    let n = DescriptionNode::new(v);
    Ok((s, Node::Description(Box::new(n))))
}

pub fn reference_parse(s: &str) -> IResult<&str, Node> {
    let (s, v) = single_statement_parse(s, String::from("reference"))?;
    let node = ReferenceNode::new(v);
    Ok((s, Node::Reference(Box::new(node))))
}

pub fn semicolon_end_parse(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, _) = tag(";")(s)?;
    Ok((s, vec![]))
}
