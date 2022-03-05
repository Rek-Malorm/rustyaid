use crate::number::{some_number_between, some_number_less_than};

const ALPHA_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
const NUMERIC_CHARSET: &[u8] = b"0123456789";

const ALPHANUMERIC_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

const ASCII_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!@#$%^&*(){}[]\"\
                            <>',.?+|_/=\\-:;`~";

const DEFAULT_MIN_LENGTH: usize = 1;
const DEFAULT_MAX_LENGTH: usize = 1024;

pub fn some_string() -> String {
    some_string_of_length(DEFAULT_MAX_LENGTH)
}

pub fn some_string_of_length(to_bound: usize) -> String {
    some_string_of_length_between(DEFAULT_MIN_LENGTH, to_bound)
}

pub fn some_string_of_length_between(from_bound: usize, to_bound: usize) -> String {
    let bound = some_number_between(from_bound, to_bound);
    string_for_charset(bound, ASCII_CHARSET)
}

pub fn some_alphanumeric_string() -> String {
    some_alphanumeric_string_of_length(DEFAULT_MAX_LENGTH)
}

pub fn some_alphanumeric_string_of_length(to_bound: usize) -> String {
    some_alphanumeric_string_of_length_between(DEFAULT_MIN_LENGTH, to_bound)
}

pub fn some_alphanumeric_string_of_length_between(from_bound: usize, to_bound: usize) -> String {
    let bound = some_number_between(from_bound, to_bound);
    string_for_charset(bound, ALPHANUMERIC_CHARSET)
}

pub fn some_numeric_string() -> String {
    some_numeric_string_of_length( DEFAULT_MAX_LENGTH)
}

pub fn some_numeric_string_of_length(to_bound: usize) -> String {
    some_numeric_string_of_length_between(DEFAULT_MIN_LENGTH, to_bound)
}

pub fn some_numeric_string_of_length_between(from_bound: usize, to_bound: usize) -> String {
    let bound = some_number_between(from_bound, to_bound);
    string_for_charset(bound, NUMERIC_CHARSET)
}

pub fn some_alpha_string() -> String {
    some_alpha_string_of_length(DEFAULT_MAX_LENGTH)
}

pub fn some_alpha_string_of_length(to_bound: usize) -> String {
    some_alpha_string_of_length_between(DEFAULT_MIN_LENGTH, to_bound)
}

pub fn some_alpha_string_of_length_between(from_bound: usize, to_bound: usize) -> String {
    let bound = some_number_between(from_bound, to_bound);
    string_for_charset(bound, ALPHA_CHARSET)
}

fn string_for_charset(bound: usize, charset: &[u8]) -> String {
    (0..bound).map(|_| {
        charset[some_number_less_than(charset.len())] as char
    })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_some_string() {
        let actual = some_string();
        assert!(actual.len() > 0);
    }

    #[test]
    fn can_create_some_string_of_length() {
        let length = some_number_between(1, 64);
        let actual = some_string_of_length(length);
        assert_eq!(actual.len(), length);
    }

    #[test]
    fn can_create_some_string_of_length_between() {
        let min_length = some_number_between(1, 32);
        let max_length = some_number_between(33, 64);
        let actual = some_string_of_length_between(min_length, max_length);
        assert!(actual.len() >= min_length);
        assert!(actual.len() <= max_length);
    }

    #[test]
    fn can_create_some_alpha_string() {
        let actual = some_alpha_string();
        assert!(actual.len() > 0);
    }

    #[test]
    fn can_create_some_alpha_string_of_length() {
        let length = some_number_between(1, 64);
        let actual = some_alpha_string_of_length(length);
        assert_eq!(actual.len(), length);
    }

    #[test]
    fn can_create_some_alpha_string_of_length_between() {
        let min_length = some_number_between(1, 32);
        let max_length = some_number_between(33, 64);
        let actual = some_alpha_string_of_length_between(min_length, max_length);
        assert!(actual.len() >= min_length);
        assert!(actual.len() <= max_length);
    }

    #[test]
    fn can_create_some_alphanumeric_string() {
        let actual = some_alphanumeric_string();
        assert!(actual.len() > 0);
    }

    #[test]
    fn can_create_some_alphanumeric_string_of_length() {
        let length = some_number_between(1, 64);
        let actual = some_alphanumeric_string_of_length(length);
        assert_eq!(actual.len(), length);
    }

    #[test]
    fn can_create_some_alphanumeric_string_of_length_between() {
        let min_length = some_number_between(1, 32);
        let max_length = some_number_between(33, 64);
        let actual = some_alphanumeric_string_of_length_between(min_length, max_length);
        assert!(actual.len() >= min_length);
        assert!(actual.len() <= max_length);
    }

    #[test]
    fn can_create_some_numeric_string() {
        let actual = some_numeric_string();
        assert!(actual.len() > 0);
    }

    #[test]
    fn can_create_some_numeric_string_of_length() {
        let length = some_number_between(1, 64);
        let actual = some_numeric_string_of_length(length);
        assert_eq!(actual.len(), length);
    }

    #[test]
    fn can_create_some_numeric_string_of_length_between() {
        let min_length = some_number_between(1, 32);
        let max_length = some_number_between(33, 64);
        let actual = some_numeric_string_of_length_between(min_length, max_length);
        assert!(actual.len() >= min_length);
        assert!(actual.len() <= max_length);
    }
}