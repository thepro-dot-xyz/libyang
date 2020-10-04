use crate::modules::*;
use crate::parser::*;
use nom::branch::{alt, permutation};
use nom::character::complete::{char, multispace0};
use nom::multi::separated_nonempty_list;
use nom::IResult;

pub fn range_uint_multi_parse(s: &str) -> IResult<&str, Vec<RangeUint>> {
    let (s, v) = separated_nonempty_list(
        permutation((multispace0, char('|'), multispace0)),
        alt((range_uint_parse, range_uint_single_parse)),
    )(s)?;
    Ok((s, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_uint_multi_parse() {
        let literal = "0..1";
        let result = range_uint_multi_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "1..20 | 22..24";
        let result = range_uint_multi_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "1..20 | 22..24 | 35..100";
        let result = range_uint_multi_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "0 | 1..10";
        let result = range_uint_multi_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);
    }
}
