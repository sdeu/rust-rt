use nom::{IResult, character::complete::{space0, space1}};
use nom::sequence::*;
use nom::bytes::complete::tag;
use nom::number::complete::float;
use nom::multi::*;
use na::{self, Isometry3};

pub fn look_at(input: &str) -> IResult<&str, Isometry3<f32>> {
    let (rest, (e, t, u)) = preceded(
        pair(tag("LookAt"), space1),
       tuple((point3, point3, vec3)))(input)?;
    Ok((rest, na::Isometry3::look_at_rh(&e, &t, &u)))
}

pub fn number(input: &str) -> IResult<&str, f32> {
    terminated(float, space0)(input)
}

pub fn vec3(input: &str) -> IResult<&str, na::Vector3<f32>> {
    let (rest, v) = count(number, 3)(input)?;
    Ok((rest, na::Vector3::new(v[0], v[1], v[2])))
}

pub fn point3(input: &str) -> IResult<&str, na::Point3<f32>> {
    let (rest, v) = count(number, 3)(input)?;
    Ok((rest, na::Point3::new(v[0], v[1], v[2])))
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
        assert_eq!(look_at("LookAt 3 4 1.5 0.5 0.5 0 0 0 1"), 
        Ok(("", iso)));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("-123.45 "), Ok(("", -123.45)))
    }

    #[test]
    fn test_vec3() {
        assert_eq!(vec3("1.0 2.0 3.0"), Ok(("", na::Vector3::new(1., 2., 3.))))
    }
}