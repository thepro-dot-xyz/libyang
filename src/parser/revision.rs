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
    let n = RevisionNode::new(std::format!("{}-{}-{}", year, month, day));
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
    let (s, _) = tag("revision")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, v) = revision_date_token_parse(s)?;
    let (s, _) = multispace0(s)?;
    let (s, subs) = alt((revision_sub_parse, semicolon_end_parse))(s)?;

    if let Node::Revision(mut node) = v {
        for sub in &subs {
            match sub {
                Node::Description(n) => {
                    node.description = Some(n.name.to_owned());
                }
                Node::Reference(n) => {
                    node.reference = Some(n.name.to_owned());
                }
                _ => {}
            }
        }
        return Ok((s, Node::Revision(node)));
    }
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
        assert_eq!(v, node);
        let (_, v) = revision_date_token_parse(revision).unwrap();
        assert_eq!(v, node);
    }

    #[test]
    fn revision_date_quoted_parse_test() {
        let revision = "\"2020-08-11\"";
        let n = RevisionNode {
            name: String::from("2020-08-11"),
            description: None,
            reference: None,
        };
        let node = Node::Revision(Box::new(n));

        let (_, v) = revision_date_quoted_parse(revision).unwrap();
        assert_eq!(v, node);
        let (_, v) = revision_date_token_parse(revision).unwrap();
        assert_eq!(v, node);
    }

    #[test]
    fn revision_statement_test() {
        let revision = r#"
        revision 2018-02-20 {
          description
            "Updated to support NMDA.";
          reference
            "RFC 8343: A YANG Data Model for Interface Management";
        }
        "#;

        let n = RevisionNode {
            name: String::from("2018-02-20"),
            description: Some(String::from("Updated to support NMDA.")),
            reference: Some(String::from(
                "RFC 8343: A YANG Data Model for Interface Management",
            )),
        };
        let node = Node::Revision(Box::new(n));

        let (_, v) = revision_parse(revision).unwrap();
        assert_eq!(v, node);
    }

    #[test]
    fn revision_single_statement_test() {
        let revision = r#"
        revision 2018-02-20;
        "#;

        let n = RevisionNode {
            name: String::from("2018-02-20"),
            description: None,
            reference: None,
        };
        let node = Node::Revision(Box::new(n));

        let (_, v) = revision_parse(revision).unwrap();
        assert_eq!(v, node);
    }
}
