use nom::IResult;
use nom::character::complete::*;
use nom::sequence::*;
use nom::bytes::complete::{tag, is_not};
use nom::number::complete::float;
use nom::multi::*;
use nom::combinator::*;
use na::{self, Isometry3};

pub fn look_at(input: &str) -> IResult<&str, Isometry3<f32>> {
    let (rest, (e, t, u)) = preceded(
        pair(tag("LookAt"), space1),
       tuple((point3, point3, vec3)))(input)?;
    Ok((rest, na::Isometry3::look_at_rh(&e, &t, &u)))
}

pub fn number(input: &str) -> IResult<&str, f32> {
    //terminated(float, space0)(input)
    ws(float)(input)
}

pub fn vec3(input: &str) -> IResult<&str, na::Vector3<f32>> {
    let (rest, v) = count(number, 3)(input)?;
    Ok((rest, na::Vector3::new(v[0], v[1], v[2])))
}

pub fn point3(input: &str) -> IResult<&str, na::Point3<f32>> {
    let (rest, v) = count(number, 3)(input)?;
    Ok((rest, na::Point3::new(v[0], v[1], v[2])))
}

pub fn peol_comment<'a>(i: &'a str) -> IResult<&'a str, ()>
{
  value(
    (), // Output is thrown away.
    pair(char('#'), is_not("\n\r"))
  )(i)
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
  where
  F: Fn(&'a str) -> IResult<&'a str, O>,
{
  delimited(
    multispace0,
    inner,
    pair(opt(multispace0), pair(opt(peol_comment), opt(multispace0)))
  )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lookat(){
        let eye = na::Point3::new(3., 4., 1.5);
        let target = na::Point3::new(0.5, 0.5, 0.);
        let up = na::Vector3::new(0., 0., 1.);
        let iso = na::Isometry3::look_at_rh(&eye, &target, &up);
        assert_eq!(look_at("LookAt 3 4 1.5 # eye \n\r 0.5 0.5 0 # target \n\r 0 0 1 # up\n\r"), 
        Ok(("", iso)));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("-123.45"), Ok(("", -123.45)))
    }

    #[test]
    fn test_vec3() {
        assert_eq!(vec3("1.0 2.0 3.0"), Ok(("", na::Vector3::new(1., 2., 3.))))
    }

    #[test]
    fn test_number_spaces() {
        assert_eq!(number("\n\r 123.45  \n\r"), Ok(("", 123.45)))
    }

    #[test]
    fn test_vec3_spaces() {
        assert_eq!(vec3("\n\r 123.45\n\r234.56\n\r345.67 # comment "), Ok(("", na::Vector3::new(123.45, 234.56, 345.67))))
    }

    #[test]
    fn test_peol_comment() {
        assert_eq!(peol_comment("# comment"), Ok(("", ())))
    }
}