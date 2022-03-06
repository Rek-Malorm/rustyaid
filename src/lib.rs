//! This crate is a simple library, providing simple methods to generate random values for different
//! Rust types.
//! This crate is inspired by the Java library [smt-random](https://github.com/shiver-me-timbers/smt-random-parent/tree/master/smt-random-numbers)
//! to help with [Mockist TDD](https://martinfowler.com/articles/mocksArentStubs.html#ClassicalAndMockistTesting)
//! # Number
//! The number module provides methods to generate numbers for any primitive numeric type
//! ## Usage
//! ```
//! use rustaid::number::*;
//!
//! let value: i32 = some_number(); // Generate some signed integer of 32 bytes
//! let value = some_number::<i32>(); // Generate some signed integer of 32 bytes alternative call
//!
//! some_number_between(5, 10); // Generate some signed integer between 5 and 10
//! some_number_less_than(9); // Generates some signed integer less or equal to 9, can be less than 0 if of signed type
//! some_number_greater_than(9); // Generates some number greater than 9
//! some_negative_number::<isize>(); // Generates some negative number
//! some_positive_number::<isize>(); // Generates some positive number
//! ```
//!
//! # Byte
//! The byte module provides methods to generate bytes, singular and ass prat of a vec
//! ## Usage
//! ```
//! use rustaid::byte::*;
//!
//! some_byte(); // Returns a signgular byte (u8)
//! some_byte_vector(1024); // Returns a vec containing 1024 random bytes
//! ```
//!
//! # String
//! The string module privodes methods to generate random strings.
//! ## Usage
//! ```
//! use rustaid::string::*;
//!
//! some_string(); // Generates a string of random length between 1 to 1024 characters
//! some_string_of_length(32); // Generates a string of 32 characters
//! some_string_of_length_between(10, 100); // Generates a string of random length between 10 and 100 characters
//!
//! some_alpha_string(); // Generates a string of random length between 1 to 1024 alphabet characters
//! some_alpha_string_of_length(32); // Generates a string of 32 alphabet characters
//! some_alpha_string_of_length_between(10, 100); // Generates a string of random length between 10 and 100 alphabet characters
//!
//! some_numeric_string(); // Generates a string of random length between 1 to 1024 numerical characters
//! some_numeric_string_of_length(32); // Generates a string of 32 numerical characters
//! some_numeric_string_of_length_between(10, 100); // Generates a string of random length between 10 and 100 numerical characters
//!
//! some_alphanumeric_string(); // Generates a string of random length between 1 to 1024 alphanumerical characters
//! some_alphanumeric_string_of_length(32); // Generates a string of 32 alphanumerical characters
//! some_alphanumeric_string_of_length_between(10, 100); // Generates a string of random length between 10 and 100 alphanumerical characters
//! ```

/// A simple random byte generator
pub mod byte;

/// A simple random number generator
pub mod number;

/// A simple random string generator
pub mod string;
