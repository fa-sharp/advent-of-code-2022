use nom::{
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1},
    sequence::terminated,
    IResult,
};

pub fn parse_decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn parse_int_decimal(input: &str) -> IResult<&str, i32> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |out: &str| i32::from_str_radix(&str::replace(&out, "_", ""), 10),
    )(input)
}

pub fn parse_usize_decimal(input: &str) -> IResult<&str, usize> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |out: &str| usize::from_str_radix(&str::replace(&out, "_", ""), 10),
    )(input)
}
