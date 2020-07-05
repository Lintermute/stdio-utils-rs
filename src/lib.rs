// Copyright 2020 Andreas Waidler
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! A minimal library that sums numbers read from a stream of strings,
//! such as `stdin`:
//!
//!     let twenty = "20";
//!     let twentytwo = "22";
//!     let stream = vec![twenty, twentytwo].into_iter();
//!     assert_eq!(stdio_utils::sum_strings(stream).unwrap(), 42);
//!
//!     let twenty = Ok(String::from("20"));
//!     let twentytwo = Ok(String::from("22"));
//!     let stream = vec![twenty, twentytwo].into_iter();
//!     assert_eq!(stdio_utils::sum(stream).unwrap(), 42);

use std::{io, num};

type Number = isize;

#[derive(thiserror::Error, Debug)]
pub enum Error
{
    #[error("Could not read input")]
    InputError(#[source] std::io::Error),

    #[error("Could not parse \"{input}\" to number")]
    ParsingError
    {
        input:  String,
        source: num::ParseIntError,
    },
}

/// Sums a stream of `String` `Result`s.
///
/// # Examples
///
///     let twenty = Ok(String::from("20"));
///     let twentytwo = Ok(String::from("22"));
///     let stream = vec![twenty, twentytwo].into_iter();
///     assert_eq!(stdio_utils::sum(stream).unwrap(), 42);
///
/// # Errors
///
/// Errors passed as input be propagated:
///
///     use std::io;
///     let input_error = io::Error::new(io::ErrorKind::Other, "Mock Error");
///     let stream = vec![Err(input_error)].into_iter();
///     stdio_utils::sum(stream).unwrap_err();
///
/// # See also
///
/// - [`sum_strings()`](fn.sum_strings.html): Variant that takes string literals

pub fn sum<T>(lines: T) -> Result<Number, Error>
where
    T: Iterator<Item = Result<String, io::Error>>,
{
    read(lines).map(|line| as_number(&line?)).sum()
}

fn read<T>(lines: T) -> impl Iterator<Item = Result<String, Error>>
where
    T: Iterator<Item = Result<String, io::Error>>,
{
    lines.map(|line| line.map_err(Error::InputError))
}

fn as_number(line: &str) -> Result<Number, Error>
{
    // We cannot use From here because ParseIntError
    // does not contain a reference to offending input.
    line.trim().parse().map_err(|source| Error::ParsingError {
        input: String::from(line),
        source,
    })
}

/// Sums a stream of string literals.
///
/// # Examples
///
///     let twenty = "20";
///     let twentytwo = "22";
///     let stream = vec![twenty, twentytwo].into_iter();
///     assert_eq!(stdio_utils::sum_strings(stream).unwrap(), 42);
///
/// # Errors
///
/// Internal errors, such as parsing errors, will be returned to the caller:
///
///     let stream = vec!["this_is_not_a_number"].into_iter();
///     stdio_utils::sum_strings(stream).unwrap_err();
///
/// # See also
///
/// - [`sum()`](fn.sum.html): Variant that takes `Result`s as parameter

pub fn sum_strings<'a, T>(strings: T) -> Result<Number, Error>
where
    T: Iterator<Item = &'a str>,
{
    sum(strings.map(|s| Ok(String::from(s))))
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn parses_a_number()
    {
        assert_eq!(as_number("42").unwrap(), 42);
    }

    #[test]
    fn parses_a_number_with_whitespace()
    {
        assert_eq!(as_number("\t 42\n").unwrap(), 42);
    }

    #[test]
    fn fails_on_invalid_character()
    {
        let result = as_number(bad_input_char());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains(bad_input_char()),
            "Offending character not part of error message \"{}\"",
            msg
        );
    }

    #[test]
    fn fails_on_empty_input()
    {
        let msg = as_number("").unwrap_err().to_string();
        assert!(
            !msg.contains(bad_input_char()),
            "Unexpected (hardcoded?) text in error message \"{}\"",
            msg
        );
    }

    #[test]
    fn empty_stream_returns_zero()
    {
        let stream = std::iter::empty();
        assert_eq!(sum(stream).unwrap(), 0);
    }

    #[test]
    fn single_element_is_equal_to_sum()
    {
        let stream = vec!["42"].into_iter();

        assert_eq!(sum_strings(stream).unwrap(), 42);
    }

    fn bad_input_char() -> &'static str
    {
        "$"
    }
}
