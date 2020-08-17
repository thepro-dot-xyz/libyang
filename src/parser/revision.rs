use crate::parser::*;
use crate::Node;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{char, multispace0, multispace1};
use nom::multi::many0;
use nom::IResult;

// YYYY-MM-DD format.
fn revision_date_parse(s: &str) -> IResult<&str, &str> {
    let (s, _year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(s)?;
    let (s, _) = char('-')(s)?;
    let (s, _month) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(s)?;
    let (s, _) = char('-')(s)?;
    let (s, _day) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(s)?;
    Ok((s, ""))
}

// Quoted "YYYY-MM-DD" format.
fn revision_date_quoted_parse(s: &str) -> IResult<&str, &str> {
    let (s, _) = char('"')(s)?;
    let (s, o) = revision_date_parse(s)?;
    let (s, _) = char('"')(s)?;
    Ok((s, o))
}

fn revision_date_token_parse(s: &str) -> IResult<&str, &str> {
    alt((revision_date_parse, revision_date_quoted_parse))(s)
}

fn revision_sub_parse(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, _) = char('{')(s)?;
    let (s, nodes) = many0(alt((description_parse, reference_parse)))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    Ok((s, nodes))
}

pub fn revision_parse(s: &str) -> IResult<&str, Node> {
    let (s, _) = multispace0(s)?;
    let (s, _k) = tag("revision")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _v) = revision_date_token_parse(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = alt((revision_sub_parse, semicolon_end_parse))(s)?;
    Ok((s, Node::EmptyNode))
}
