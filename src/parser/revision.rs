use crate::modules::RevisionNode;
use crate::parser::*;
use crate::Node;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{char, multispace0, multispace1};
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;

// YYYY-MM-DD format.
fn revision_date_parse(s: &str) -> IResult<&str, Node> {
    let (s, year) = take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(s)?;
    let (s, _) = char('-')(s)?;
    let (s, month) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(s)?;
    let (s, _) = char('-')(s)?;
    let (s, day) = take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(s)?;
    let mut n = RevisionNode::default();
    n.name = std::format!("{}-{}-{}", year, month, day);
    Ok((s, Node::Revision(Box::new(n))))
}

// Quoted "YYYY-MM-DD" format.
fn revision_date_quoted_parse(s: &str) -> IResult<&str, Node> {
    let (s, n) = delimited(char('"'), revision_date_parse, char('"'))(s)?;
    Ok((s, n))
}

fn revision_date_token_parse(s: &str) -> IResult<&str, Node> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revision_date_parse_test() {
        let revision = "2020-08-10";
        let n = RevisionNode {
            name: String::from("2020-08-10"),
            description: None,
            reference: None,
        };
        let node = Node::Revision(Box::new(n));

        let (_, v) = revision_date_parse(revision).unwrap();
        println!("XXX {:?}", v);
        println!("XXX {:?}", node);
        assert_eq!(v, node);

        println!("{:?}", revision_date_token_parse(revision));
    }

    #[test]
    fn revision_date_quoted_parse_test() {
        let revision = "\"2020-08-11\"";
        println!("{:?}", revision_date_quoted_parse(revision));
        println!("{:?}", revision_date_token_parse(revision));
    }

    #[test]
    fn revision_single_statement_test() {}
}
