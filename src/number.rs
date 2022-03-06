use std::ops::{Add, Sub};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use num::{Bounded, FromPrimitive, Signed};
use rand::distributions::uniform::SampleUniform;

/// Generates a new random number
///
/// Generates a new number between the min and max values for the numerical type.
/// Supports any primitive numeric type with finite bounds
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::number::*;
/// let n: i32 = some_number();
/// ```
pub fn some_number<TYPE>() -> TYPE where TYPE: Bounded, Standard: Distribution<TYPE> {
    let mut rng = rand::thread_rng();
    rng.gen()
}

/// Generates a new random positive number
///
/// Generates a new number between the `0` and max values for the numerical type.
/// Supports any primitive numeric type with finite bounds
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::number::*;
/// let n: i32 = some_positive_number();
/// ```
pub fn some_positive_number<TYPE>() -> TYPE
    where TYPE: Bounded + FromPrimitive + PartialOrd + Add<Output=TYPE>,
          Standard: Distribution<TYPE> {
    let number = some_number();
    if number <= TYPE::from_i8(0).unwrap() {
        number.add(TYPE::max_value())
    } else {
        number
    }
}

/// Generates a new random negative number
///
/// Generates a new number between the min and `0` for the numerical type.
/// Supports any primitive numeric signed type with finite bounds
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::number::*;
/// let n: i32 = some_negative_number();
/// ```
pub fn some_negative_number<TYPE>() -> TYPE
    where TYPE: Bounded + FromPrimitive + PartialOrd + Sub<Output=TYPE> + Signed,
          Standard: Distribution<TYPE> {
    let number = some_number();
    if number >= TYPE::from_i8(0).unwrap() {
        number.neg()
    } else {
        number
    }
}

/// Generates a new random number between two values
///
/// Generates a new number between the min and max values provided for the numerical type.
/// Supports any primitive numeric type with finite bounds
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::number::*;
/// let n: i32 = some_number_between(10, 20);
/// ```
pub fn some_number_between<TYPE>(from: TYPE, to: TYPE) -> TYPE
    where TYPE: FromPrimitive + PartialOrd + Sub<Output=TYPE>
    + SampleUniform, Standard: Distribution<TYPE> {
    if from == to{
        from
    }
    else {
        let mut rand_generator = rand::thread_rng();
        rand_generator.gen_range(from..to)
    }
}

/// Generates a new random number greater than some value
///
/// Generates a new number between the provided minimum and the max value for the numerical type
/// Supports any primitive numeric type with finite bounds
/// Only supports generating positive numbers
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::number::*;
/// let n: i32 = some_number_greater_than(10);
/// ```
pub fn some_number_greater_than<TYPE>(bound: TYPE) -> TYPE
    where TYPE: Bounded + FromPrimitive + PartialOrd + Add<Output = TYPE>
    + Sub<Output=TYPE> + SampleUniform, Standard: Distribution<TYPE> {
    let zero = TYPE::from_i8(0).unwrap();
    if bound < zero {
        panic!("Cannot handle negative value")
    }
    some_number_between(bound + TYPE::from_i8(1).unwrap(), TYPE::max_value())
}

/// Generates a new random number less than some value
///
/// Generates a new number between the the minimum value for the numerical type and the provided max value
/// Supports any primitive numeric type with finite bounds
/// Supports signed and unsigned types
/// Can generate negative values
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::number::*;
/// let n: i32 = some_number_less_than(10);
/// ```
pub fn some_number_less_than<TYPE>(bound: TYPE) -> TYPE
    where TYPE: Bounded + FromPrimitive + PartialOrd + Add<Output = TYPE>
    + Sub<Output=TYPE> + SampleUniform, Standard: Distribution<TYPE> {
    some_number_between(TYPE::min_value(), bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_random_signed_integers() {
        let _ = some_number::<i8>();
        let _ = some_number::<i16>();
        let _ = some_number::<i32>();
        let _ = some_number::<i64>();
        let _ = some_number::<i128>();
        let _ = some_number::<isize>();
    }

    #[test]
    fn can_generate_random_unsigned_integers() {
        let _ = some_number::<u8>();
        let _ = some_number::<u16>();
        let _ = some_number::<u32>();
        let _ = some_number::<u64>();
        let _ = some_number::<u128>();
        let _ = some_number::<usize>();
    }

    #[test]
    fn can_generate_random_signed_floats() {
        let _ = some_number::<f32>();
        let _ = some_number::<f64>();
    }

    #[test]
    fn can_generate_1000_positive_usize() {
        for _ in 0..1000 {
            let actual = some_positive_number::<usize>();
            assert!(actual > 0);
        }
    }

    #[test]
    fn can_generate_signed_integer_between() {
        let from = -1000;
        let to = 1000;
        let actual = some_number_between(from, to);
        assert!(actual >= from);
        assert!(actual < to);
    }

    #[test]
    fn can_generate_unsigned_integer_between() {
        let from: u32 = 0;
        let to: u32 = 1000;
        let actual = some_number_between(from, to);
        assert!(actual >= from);
        assert!(actual < to);
    }

    #[test]
    fn can_generate_signed_number_less_than() {
        let to: u32 = 1000;
        let actual = some_number_less_than(to);
        assert!(actual < to);
    }

    #[test]
    fn can_generate_signed_number_greater_than() {
        let from: u32 = 1000;
        let actual = some_number_greater_than(from);
        assert!(actual > from);
    }
}