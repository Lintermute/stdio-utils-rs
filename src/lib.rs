type Number = isize;

#[derive(Debug, PartialEq)]
pub enum Error {
    ReadingError,
    ParsingError,
}

pub fn sum<T>(lines: T) -> Result<Number, Error>
where
    T: Iterator<Item = String>,
{
    lines
        .map(|s| as_number(&s))
        .sum()
}

pub fn as_number(line: &str) -> Result<Number, Error> {
    match line.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(Error::ParsingError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_number() {
        assert_eq!(Ok(42), as_number("42"));
    }

    #[test]
    fn parses_a_number_with_whitespace() {
        assert_eq!(Ok(42), as_number("\t42 \n"));
    }

    #[test]
    fn fails_on_empty_input() {
        match as_number("") {
            Ok(v) => panic!("Empty string was parsed to value {}", v),
            Err(e) => println!("Got an expected error: {:?}", e),
        }
    }

    #[test]
    fn empty_stream_returns_zero() {
        assert_eq!(Ok(0), sum(std::iter::empty()));
    }

    #[test]
    fn single_element_is_its_sum() {
        let v = vec!["42".to_string()];

        assert_eq!(Ok(42), sum(v.into_iter()));
    }

    #[test]
    fn sums_two_elements() {
        let v = vec![
            "20".to_string(),
            "22".to_string()
        ];

        assert_eq!(Ok(42), sum(v.into_iter()));
    }
}
