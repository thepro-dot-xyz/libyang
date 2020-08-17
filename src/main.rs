use libyang::*;

// use escape8259::unescape;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::multi::many0;
use nom::IResult;

// Module:top
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
            let n = NamespaceNode::new(v);
            Node::Namespace(Box::new(n))
        }
        "prefix" => {
            let n = PrefixNode::new(v);
            Node::Prefix(Box::new(n))
        }
        "organization" => {
            let n = OrganizationNode::new(v);
            Node::Organization(Box::new(n))
        }
        "contact" => {
            let n = ContactNode::new(v);
            Node::Contact(Box::new(n))
        }
        "description" => {
            let n = DescriptionNode::new(v);
            Node::Description(Box::new(n))
        }
        _ => Node::EmptyNode,
    };
    Ok((s, node))
}

fn yang_parse(s: &str) -> IResult<&str, Module> {
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
                module.namespace = n.name.clone();
            }
            Node::Prefix(n) => {
                module.prefix = n.name.clone();
            }
            Node::Organization(n) => {
                module.organization = Some(n.name.clone());
            }
            //     "contact" => {
            //         module.contact = Some(String::from(v));
            //     }
            //     "description" => {
            //         module.description = Some(String::from(v));
            //     }
            _ => {}
        }
    }
    Ok((s, module))
}

fn main() {
    // Allocate a new Yang.
    let mut yang = Yang::new();
    yang.add_path("/etc/openconfigd/yang:yang/...");

    // Read a module "ietf-inet-types".
    let mut ms = Modules::new();
    let data = yang.read(&ms, "ietf-inet-types").unwrap();

    match yang_parse(&data) {
        Ok((_, module)) => {
            println!("Module name: {}", module.name);
            println!("Module namespace: {}", module.namespace);
            println!("Module prefix: {}", module.prefix);
            ms.modules.insert(module.prefix.clone(), module);

            let entry = ms.modules.get(&"inet".to_string());
            if let Some(e) = entry {
                println!("XXX found {:?}", e);
            } else {
                println!("XXX not found")
            }
        }
        Err(e) => {
            println!("module parse: {:?}", e);
        }
    }

    // let ytype = YangType::new(TypeKind::Yenum);
    // println!("{:?}", ytype);

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
