use nom::{
    character::complete::digit1,
    combinator::map_res,
    IResult,
};

pub fn parse_decimal_digits(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

pub fn parse_int_decimal(input: &str) -> IResult<&str, i32> {
    map_res(
        parse_decimal_digits,
        |out: &str| i32::from_str_radix(&out, 10),
    )(input)
}

pub fn parse_u64_decimal(input: &str) -> IResult<&str, u64> {
    map_res(
        parse_decimal_digits,
        |out: &str| u64::from_str_radix(&out, 10),
    )(input)
}

pub fn parse_usize_decimal(input: &str) -> IResult<&str, usize> {
    map_res(
        parse_decimal_digits,
        |out: &str| usize::from_str_radix(&out, 10),
    )(input)
}
