use libyang::{Modules, Yang};

use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{anychar, char, multispace0, multispace1};
use nom::combinator::{recognize, verify};
use nom::sequence::pair;
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

#[inline]
pub fn is_identifier(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.'
}

pub fn identifier(s: &str) -> IResult<&str, &str> {
    recognize(pair(
        verify(anychar, |c: &char| c.is_ascii_alphabetic() || c == &'_'),
        take_while(is_identifier),
    ))(s)
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
}
