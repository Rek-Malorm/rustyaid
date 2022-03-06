use crate::number::{some_number_between, some_number_less_than};

const ALPHA_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz ";
const NUMERIC_CHARSET: &[u8] = b"0123456789";

const ALPHANUMERIC_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789 ";

const ASCII_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!@#$%^&*(){}[]\"\
                            <>',.?+|_/=\\-:;`~ ";

const DEFAULT_MIN_LENGTH: usize = 1;
const DEFAULT_MAX_LENGTH: usize = 1024;

/// Creates a new string of random characters
///
/// Creates a new string of a random length between 1 and 1024 characters.
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as a space and the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
/// >
/// > 0123456789!@#$%^&*(){}[]\"
/// >
/// > <>',.?+|_/=\\-:;`~
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_string();
/// ```
pub fn some_string() -> String {
    some_string_of_length_between(DEFAULT_MIN_LENGTH,DEFAULT_MAX_LENGTH)
}

/// Creates a new string of random characters for a set length
///
/// Creates a new string of `to_bound: usize` characters.
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as a space and the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
/// >
/// > 0123456789!@#$%^&*(){}[]\"
/// >
/// > <>',.?+|_/=\\-:;`~
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_string_of_length(50);
/// ```
pub fn some_string_of_length(to_bound: usize) -> String {
    some_string_of_length_between(to_bound, to_bound)
}

/// Creates a new string of random characters for a length between two values
///
/// Creates a new string of random characters between `from_bound: usize` and `to_bound: usize`
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as a space and the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
/// >
/// > 0123456789!@#$%^&*(){}[]\"
/// >
/// > <>',.?+|_/=\\-:;`~
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_string_of_length_between(50, 100);
/// ```
pub fn some_string_of_length_between(from_bound: usize, to_bound: usize) -> String {
    let bound = some_number_between(from_bound, to_bound);
    string_for_charset(bound, ASCII_CHARSET)
}

/// Creates a new string of random alphanumeric characters
///
/// Creates a new string of a random length between 1 and 1024 alphanumeric characters.
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as a space and the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
/// >
/// > 0123456789
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_alphanumeric_string();
/// ```
pub fn some_alphanumeric_string() -> String {
    some_alphanumeric_string_of_length_between(DEFAULT_MIN_LENGTH, DEFAULT_MAX_LENGTH)
}

/// Creates a new string of random alphanumeric characters for a set length
///
/// Creates a new string of `to_bound: usize` alphanumeric characters.
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as a space and the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
/// >
/// > 0123456789
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_alphanumeric_string_of_length(50);
/// ```
pub fn some_alphanumeric_string_of_length(to_bound: usize) -> String {
    some_alphanumeric_string_of_length_between(to_bound, to_bound)
}

/// Creates a new string of random alphanumeric characters for a length between two values
///
/// Creates a new string of random alphanumeric characters between `from_bound: usize` and `to_bound: usize`
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as a space and the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
/// >
/// > 0123456789
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_alpha_string_of_length_between(50, 100);
/// ```
pub fn some_alphanumeric_string_of_length_between(from_bound: usize, to_bound: usize) -> String {
    let bound = some_number_between(from_bound, to_bound);
    string_for_charset(bound, ALPHANUMERIC_CHARSET)
}

/// Creates a new string of random numeric characters
///
/// Creates a new string of a random length between 1 and 1024 numeric characters.
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as the following
/// > 0123456789
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_numeric_string();
/// ```
pub fn some_numeric_string() -> String {
    some_numeric_string_of_length_between( DEFAULT_MIN_LENGTH, DEFAULT_MAX_LENGTH)
}

/// Creates a new string of random numeric characters for a set length
///
/// Creates a new string of `to_bound: usize` numeric characters.
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as the following
/// > 0123456789
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_numeric_string_of_length(50);
/// ```
pub fn some_numeric_string_of_length(to_bound: usize) -> String {
    some_numeric_string_of_length_between(to_bound, to_bound)
}

/// Creates a new string of random numeric characters for a length between two values
///
/// Creates a new string of random numeric characters between `from_bound: usize` and `to_bound: usize`
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as the following
/// > 0123456789
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_numeric_string_of_length_between(50, 100);
/// ```
pub fn some_numeric_string_of_length_between(from_bound: usize, to_bound: usize) -> String {
    let bound = some_number_between(from_bound, to_bound);
    string_for_charset(bound, NUMERIC_CHARSET)
}

/// Creates a new string of random alpha characters
///
/// Creates a new string of a random length between 1 and 1024 alpha characters.
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_numeric_string();
/// ```
pub fn some_alpha_string() -> String {
    some_alpha_string_of_length_between(DEFAULT_MIN_LENGTH, DEFAULT_MAX_LENGTH)
}

/// Creates a new string of random alpha characters for a set length
///
/// Creates a new string of `to_bound: usize` alpha characters.
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_alpha_string_of_length(50);
/// ```
pub fn some_alpha_string_of_length(to_bound: usize) -> String {
    some_alpha_string_of_length_between(to_bound, to_bound)
}

/// Creates a new string of random alpha characters for a length between two values
///
/// Creates a new string of random alpha characters between `from_bound: usize` and `to_bound: usize`
/// Each character can be any ASCII value with a valid character value (Excluding values such as
/// NUL, BELL, DEL) and does not include the extended ASCII character set.
/// Complete set is defined as the following
/// > ABCDEFGHIJKLMNOPQRSTUVWXYZ
/// >
/// > abcdefghijklmnopqrstuvwxyz
///
/// # Examples
///
/// Basic usage:
/// ```
/// use rustaid::string::*;
/// let s = some_alpha_string_of_length_between(50, 100);
/// ```
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