use libyang::{Modules, Yang};

use escape8259::unescape;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::{anychar, char, multispace0, multispace1};
use nom::combinator::{map_res, recognize, verify};
use nom::multi::many0;
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
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C)
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

fn double_quoted_string(s: &str) -> IResult<&str, String> {
    let parser = delimited(tag("\""), string_body, tag("\""));
    map_res(parser, |x| unescape(x))(s)
}

#[allow(dead_code)]
fn namespace_parse(s: &str) -> IResult<&str, &str> {
    let (s, _) = tag("namespace")(s)?;
    let (s, _) = multispace1(s)?;
    // let (s, namespace) = escape(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, "x"))
}

fn module_parse(s: &str) -> IResult<&str, &str> {
    let (s, _) = tag("module")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, ident) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    Ok((s, ident))
}

fn main() {
    // Allocate a new Yang.
    let mut yang = Yang::new();
    yang.add_path("/etc/openconfigd/yang:tests/...");
    println!("{:?}", yang.paths());

    // Read a module "ietf-dhcp".
    let ms = Modules::new();
    let data = yang.read(&ms, "ietf-inet-types").unwrap();

    println!("{}", data);

    let literal = "module ietf-inet-types {";

    match module_parse(literal) {
        Ok(v) => {
            println!("{:?}", v);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let literal = "\"urn:ietf:params:xml:ns:yang:ietf-inet-types\"";
    println!("{}", literal);

    let literal = r"\na";
    let result = escape_code(literal);
    println!("{:?}", result);

    let literal = r#"main-routine_1 "#;
    let result = nonescaped_string(literal);
    println!("{:?}", result);

    let literal = r#""hoge hoga\n hoge""#;
    println!("l: {:?}", literal);
    match double_quoted_string(literal) {
        Ok((_, o)) => {
            println!("output: {}", o);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    //println!("{:?}", v);
}
