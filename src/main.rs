use libyang::{Modules, Yang};

// use escape8259::unescape;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1, take_while_m_n};
use nom::character::complete::{anychar, char, multispace0, multispace1};
// use nom::character::is_digit;
// use nom::combinator::{map_res, recognize, verify};
use nom::combinator::{recognize, verify};
use nom::multi::{many0, many1};
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

fn module_parse(s: &str) -> IResult<&str, &str> {
    let (s, _) = multispace0(s)?;
    let (s, m) = alt((
        tag("namespace"),
        tag("prefix"),
        tag("organization"),
        tag("contact"),
        tag("description"),
    ))(s)?;
    println!("keyword: {}", m);
    let (s, _) = multispace1(s)?;
    let (s, o) = double_quoted_string(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, o))
}

fn yang_parse(s: &str) -> IResult<&str, &str> {
    let (s, _) = tag("module")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, ident) = identifier(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('{')(s)?;
    let (s, _) = many1(module_parse)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
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

    match yang_parse(&data) {
        Ok((_, o)) => {
            println!("Module {:?} parse success", o);
        }
        Err(e) => {
            println!("module parse: {:?}", e);
        }
    }

    let revision = "2020-08-10";
    println!("{:?}", revision_date_parse(revision));
    let revision_q = "\"2020-08-10\"";
    println!("{:?}", revision_date_quoted_parse(revision_q));

    println!("{:?}", revision_date_token_parse(revision));
    println!("{:?}", revision_date_token_parse(revision_q));

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
