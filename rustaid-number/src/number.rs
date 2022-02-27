use std::ops::{Add, Sub};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use num::{Bounded, FromPrimitive, Signed};
use rand::distributions::uniform::SampleUniform;

pub fn some_number<TYPE>() -> TYPE where TYPE: Bounded, Standard: Distribution<TYPE> {
    let mut rand_generator = rand::thread_rng();
    rand_generator.gen()
}

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

pub fn some_number_between<TYPE>(from: TYPE, to: TYPE) -> TYPE
    where TYPE: FromPrimitive + PartialOrd + Sub<Output=TYPE>
    + SampleUniform, Standard: Distribution<TYPE> {
    let mut rand_generator = rand::thread_rng();
    rand_generator.gen_range(from..to)
}

pub fn some_positive_number_to<TYPE>(bound: TYPE) -> TYPE
    where TYPE: FromPrimitive + PartialOrd
    + Sub<Output=TYPE> + SampleUniform, Standard: Distribution<TYPE> {
    let zero = TYPE::from_i8(0).unwrap();
    if bound <= zero {
        panic!("Cannot handle negative value")
    }
    some_number_between(zero, bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_random_signed_integers() -> Result<(), String> {
        let _ = some_number::<i8>();
        let _ = some_number::<i16>();
        let _ = some_number::<i32>();
        let _ = some_number::<i64>();
        let _ = some_number::<i128>();
        let _ = some_number::<isize>();
        Ok(())
    }

    #[test]
    fn can_generate_random_unsigned_integers() -> Result<(), String> {
        let _ = some_number::<u8>();
        let _ = some_number::<u16>();
        let _ = some_number::<u32>();
        let _ = some_number::<u64>();
        let _ = some_number::<u128>();
        let _ = some_number::<usize>();
        Ok(())
    }

    #[test]
    fn can_generate_random_signed_floats() -> Result<(), String> {
        let _ = some_number::<f32>();
        let _ = some_number::<f64>();
        Ok(())
    }

    #[test]
    fn can_generate_1000_positive_usize() -> Result<(), String> {
        for _ in 0..1000 {
            let actual = some_positive_number::<usize>();
            assert!(actual > 0);
        }
        Ok(())
    }

    #[test]
    fn can_generate_signed_integer_between() -> Result<(), String> {
        let from = -1000;
        let to = 1000;
        let actual = some_number_between(from, to);
        assert!(actual >= from);
        assert!(actual < to);
        Ok(())
    }

    #[test]
    fn can_generate_unsigned_integer_between() -> Result<(), String> {
        let from: u32 = 0;
        let to: u32 = 1000;
        let actual = some_number_between(from, to);
        assert!(actual >= from);
        assert!(actual < to);
        Ok(())
    }
}