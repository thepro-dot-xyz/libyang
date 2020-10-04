use crate::modules::*;
use crate::parser::*;
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};
use nom::multi::separated_nonempty_list;
use nom::IResult;

fn range_uint_pair_parse(s: &str) -> IResult<&str, RangeUint> {
    let (s, _) = multispace0(s)?;
    let (s, r1) = alt((tag("min"), uint_parse))(s)?;
    let (_, start) = uint_value_parse(r1)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("..")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, r2) = alt((tag("max"), uint_parse))(s)?;
    let (_, end) = uint_value_parse(r2)?;
    let range = RangeUint {
        start: start,
        end: end,
    };
    Ok((s, range))
}

fn range_uint_single_parse(s: &str) -> IResult<&str, RangeUint> {
    let (s, _) = multispace0(s)?;
    let (s, r) = alt((tag("min"), tag("max"), uint_parse))(s)?;
    let (_, val) = uint_value_parse(r)?;
    let (s, _) = multispace0(s)?;
    let range = RangeUint {
        start: val,
        end: RangeVal::None,
    };
    Ok((s, range))
}

// Parse multiple occurance of single value or range.  "0 | 2..10 | max"
pub fn range_uint_parse(s: &str) -> IResult<&str, Vec<RangeUint>> {
    let (s, v) = separated_nonempty_list(
        permutation((multispace0, char('|'), multispace0)),
        alt((range_uint_pair_parse, range_uint_single_parse)),
    )(s)?;
    Ok((s, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_uint_multi_parse() {
        let literal = "0..1";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "1..20 | 22..24";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "1..20 | 22..24 | 35..100";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "0 | 1..10";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);
    }

    #[test]
    fn test_uint_single_parse() {
        let literal = "128";
        let result = range_uint_single_parse(literal);
        println!("XXX range_uint_single_parse: {:?}", result);

        let literal = "max";
        let result = range_uint_single_parse(literal);
        println!("XXX range_uint_single_parse: {:?}", result);
    }
}
