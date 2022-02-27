use crate::number::*;

pub fn some_byte() -> u8 {
    some_number()
}

pub fn some_byte_vector(bound: usize) -> Vec<u8> {
    let mut vec = Vec::new();
    for _ in 0..bound {
        vec.push(some_number())
    }
    vec
}

#[cfg(test)]
mod tests {
    use crate::number::*;
    use super::*;

    #[test]
    fn can_create_random_byte() {
        let _byte = some_byte();
    }

    #[test]
    fn can_create_random_byte_vector() {
        let bound = some_number_between(1, 1024);
        let byte_vector = some_byte_vector(bound);
        assert_eq!(byte_vector.len(), bound);
    }
}