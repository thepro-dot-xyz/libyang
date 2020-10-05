use crate::modules::*;
use crate::parser::*;
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};
use nom::multi::separated_nonempty_list;
use nom::IResult;

// Parse pair of value for range such as "min..100".
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

// Parse single range parameter such as "min", "100".
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

// Combine single and pair parser for range with '|'. "0 | 2..10 | max"
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
    fn test_range_uint_single_parse() {
        let literal = "128";
        let result = range_uint_single_parse(literal);
        let expect = RangeUint {
            start: RangeVal::Val(128u64),
            end: RangeVal::None,
        };
        assert_eq!(result, Ok(("", expect)));

        let literal = "max";
        let result = range_uint_single_parse(literal);
        let expect = RangeUint {
            start: RangeVal::Max,
            end: RangeVal::None,
        };
        assert_eq!(result, Ok(("", expect)));

        let literal = "0";
        let result = range_uint_single_parse(literal);
        let expect = RangeUint {
            start: RangeVal::Val(0u64),
            end: RangeVal::None,
        };
        assert_eq!(result, Ok(("", expect)));

        // "-0" should fail.
        let literal = "-0";
        let result = range_uint_single_parse(literal);
        println!("{:?}", result);

        // "-100" should fail.
        let literal = "-100";
        let result = range_uint_single_parse(literal);
        println!("{:?}", result);

        // "abc" should fail.
        let literal = "abc";
        let result = range_uint_single_parse(literal);
        println!("{:?}", result);
    }

    #[test]
    fn test_range_uint_pair_parse() {
        let literal = "0..1";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);
    }

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
}
