use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, line_ending},
    combinator::{map, opt, recognize},
    multi::many1,
    sequence::{terminated, tuple},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl From<Point3D> for Vec<i64> {
    #[inline]
    fn from(point: Point3D) -> Self {
        vec![point.x, point.y, point.z]
    }
}

impl From<Point3D> for [i64; 3] {
    #[inline]
    fn from(point: Point3D) -> Self {
        [point.x, point.y, point.z]
    }
}

fn parse_integer(input: &str) -> IResult<&str, i64> {
    let (input, sign) = opt(char('-'))(input)?;
    let (input, num) = digit1(input)?;
    let value: i64 = num.parse().unwrap();
    let value = if sign.is_some() { -value } else { value };
    Ok((input, value))
}

fn parse_point3d(input: &str) -> IResult<&str, Point3D> {
    let (input, x) = parse_integer(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y) = parse_integer(input)?;
    let (input, _) = char(',')(input)?;
    let (input, z) = parse_integer(input)?;

    Ok((input, Point3D { x, y, z }))
}

fn parse_point3d_line(input: &str) -> IResult<&str, Point3D> {
    terminated(parse_point3d, line_ending)(input)
}

pub fn parse_points(input: &str) -> IResult<&str, Vec<Point3D>> {
    many1(parse_point3d_line)(input)
}
