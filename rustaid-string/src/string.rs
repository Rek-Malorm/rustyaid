use rustaid_number::number::*;

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
const DEFAULT_MAX_LENGTH: usize = 1014;

pub fn some_string() -> String {
    let bound = some_number_between(DEFAULT_MIN_LENGTH, DEFAULT_MAX_LENGTH);
    string_for_charset(bound, ASCII_CHARSET)
}

pub fn some_string_of_length(to_bound: usize) -> String {
    let bound = some_number_between(DEFAULT_MIN_LENGTH, to_bound);
    string_for_charset(bound, ASCII_CHARSET)
}

pub fn some_string_of_length_between(from_bound: usize, to_bound: usize) -> String {
    let bound = some_number_between(from_bound, to_bound);
    string_for_charset(bound, ASCII_CHARSET)
}

pub fn some_alphanumeric_string() -> String {
    let bound = some_number_between(DEFAULT_MIN_LENGTH, DEFAULT_MAX_LENGTH);
    string_for_charset(bound, ALPHANUMERIC_CHARSET)
}

pub fn some_numeric_string() -> String {
    let bound = some_number_between(DEFAULT_MIN_LENGTH, DEFAULT_MAX_LENGTH);
    string_for_charset(bound, NUMERIC_CHARSET)
}

pub fn some_alpha_string() -> String {
    let bound = some_number_between(DEFAULT_MIN_LENGTH, DEFAULT_MAX_LENGTH);
    string_for_charset(bound, ALPHA_CHARSET)
}

fn string_for_charset(bound: usize, charset: &[u8]) -> String{
    (0..bound).map(|_| {
        charset[some_positive_number_to(charset.len())] as char
    })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_some_alphanumeric_string() {
        let actual = some_alphanumeric_string();
        assert!(actual.len() > 0);
    }
}