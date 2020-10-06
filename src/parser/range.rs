use crate::modules::*;
use crate::parser::*;
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};
use nom::multi::separated_nonempty_list;
use nom::IResult;

pub fn int_value_parse(s: &str) -> IResult<&str, RangeVal<i64>> {
    match s {
        "max" => Ok((s, RangeVal::<i64>::Max)),
        "min" => Ok((s, RangeVal::<i64>::Min)),
        v => {
            let n = v.parse::<i64>().unwrap();
            Ok((s, RangeVal::<i64>::Val(n)))
        }
    }
}

fn uint_value_parse(s: &str) -> IResult<&str, RangeVal<u64>> {
    match s {
        "max" => Ok((s, RangeVal::<u64>::Max)),
        "min" => Ok((s, RangeVal::<u64>::Min)),
        v => {
            let n = v.parse::<u64>().unwrap();
            Ok((s, RangeVal::<u64>::Val(n)))
        }
    }
}

fn range_int_single_parse(s: &str) -> IResult<&str, RangeInt> {
    let (s, _) = multispace0(s)?;
    let (s, r) = alt((tag("min"), tag("max"), int_parse))(s)?;
    let (_, val) = int_value_parse(r)?;
    let (s, _) = multispace0(s)?;
    let range = RangeInt {
        start: val,
        end: RangeVal::None,
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

// Parse pair of value for range such as "min..100".
fn range_int_pair_parse(s: &str) -> IResult<&str, RangeInt> {
    let (s, _) = multispace0(s)?;
    let (s, r1) = alt((tag("min"), int_parse))(s)?;
    let (_, start) = int_value_parse(r1)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("..")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, r2) = alt((tag("max"), int_parse))(s)?;
    let (_, end) = int_value_parse(r2)?;
    let range = RangeInt {
        start: start,
        end: end,
    };
    Ok((s, range))
}

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

// Combine single and pair parser for range with '|'. "0 | 2..10 | max"
pub fn range_int_parse(s: &str) -> IResult<&str, Vec<RangeInt>> {
    let (s, v) = separated_nonempty_list(
        permutation((multispace0, char('|'), multispace0)),
        alt((range_int_pair_parse, range_int_single_parse)),
    )(s)?;
    Ok((s, v))
}

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
    use nom::error::ErrorKind;
    use nom::Err::Error;

    #[test]
    fn test_range_int_single_parse() {
        let literal = "100";
        let result = int_value_parse(literal);
        println!("{:?}", result);

        let literal = "-0";
        let result = int_value_parse(literal);
        println!("{:?}", result);

        let literal = "-1";
        let result = int_value_parse(literal);
        println!("{:?}", result);
    }

    #[test]
    fn test_range_uint_single_parse() {
        struct Test {
            input: &'static str,
            output: IResult<&'static str, RangeUint>,
        };
        let tests = [
            Test {
                input: "128",
                output: Ok((
                    "",
                    RangeUint {
                        start: RangeVal::Val(128u64),
                        end: RangeVal::None,
                    },
                )),
            },
            Test {
                input: "max",
                output: Ok((
                    "",
                    RangeUint {
                        start: RangeVal::Max,
                        end: RangeVal::None,
                    },
                )),
            },
            Test {
                input: "0",
                output: Ok((
                    "",
                    RangeUint {
                        start: RangeVal::Val(0u64),
                        end: RangeVal::None,
                    },
                )),
            },
            Test {
                input: "-0",
                output: Err(Error(("-0", ErrorKind::OneOf))),
            },
            Test {
                input: "-100",
                output: Err(Error(("-100", ErrorKind::OneOf))),
            },
            Test {
                input: "abc",
                output: Err(Error(("abc", ErrorKind::OneOf))),
            },
        ];
        for t in &tests {
            let result = range_uint_single_parse(t.input);
            assert_eq!(result, t.output);
        }
    }

    #[test]
    fn test_range_uint_pair_parse() {
        let literal = "0..1";
        let result = range_uint_parse(literal);
        let expect = RangeUint {
            start: RangeVal::Val(0u64),
            end: RangeVal::Val(1u64),
        };
        assert_eq!(result, Ok(("", vec![expect])));

        let literal = "1..100";
        let result = range_uint_parse(literal);
        let expect = RangeUint {
            start: RangeVal::Val(1u64),
            end: RangeVal::Val(100u64),
        };
        assert_eq!(result, Ok(("", vec![expect])));

        // We interpret start > end as valid statement. Although it won't match
        // to any value.
        let literal = "100..1";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "-0..1";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "-1..1";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);

        let literal = "-100..-1";
        let result = range_uint_parse(literal);
        println!("XXX range_uint_multi: {:?}", result);
    }

    #[test]
    fn test_range_uint_multi_parse() {
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
    fn parse_minus_zero() {
        let literal = "-0";
        let n = literal.parse::<i64>().unwrap();
        assert_eq!(n, 0);
    }
}
