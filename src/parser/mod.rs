use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Error as IOError;
use std::path::Path;

/// Attempts to open and parse a file line by line into a collection through a callback function.
///
/// The callback function will be used as a map and receive Result<string> as parameter.
/// The type should be compatible with collect to build a collection.
pub fn parse_file_by_line<P, T, C>(filename: P, callback: fn(Result<String, IOError>) -> T) -> Result<C, IOError>
    where P: AsRef<Path>, C: FromIterator<T> {
    let fd = File::open(filename)?;
    let lines = BufReader::new(fd).lines();

    Ok(lines
        .map(callback)
        .collect())
}

/// Attempts to open and parse a file composed of blocks of lines,
/// with a callback receiving a vector of n lines of text.
///
/// The callback function will be used as a map and receive a Vec<String> as parameter.
/// The vector will contain <lines_per_block> strings.
///
/// The type should be compatible with collect to build a vector of your collection type.
///
/// Function returns a type collected from an iterator yielding T, for example Vec<T>.
/// T is the type returned by your callback method.
pub fn parse_file_by_lines_block<P, T, C>(filename: P, lines_per_block: usize, callback: fn(Vec<String>) -> T) -> Result<C, IOError>
where P: AsRef<Path>, C: FromIterator<T> {
    let fd = File::open(filename)?;
    let lines = BufReader::new(fd).lines();

    let mut items = Vec::new();
    let mut block_lines = Vec::new();
    for line in lines {
        block_lines.push(line?);

        if block_lines.len() == lines_per_block {
            items.push(callback(block_lines));
            block_lines = Vec::new();
        }
    }

    Ok(items.into_iter().collect())
}

/// Attempts to open and parse a file composed of blocks of lines,
/// with a callback receiving a vector of n lines of text.
///
/// Each block is separated by <separator_lines_between_block> lines to ignore.
///
/// The callback function will be used as a map and receive a Vec<String> as parameter.
/// The vector will contain <lines_per_block> strings.
///
/// The type should be compatible with collect to build a vector of your collection type.
///
/// Function returns a type collected from an iterator yielding T, for example Vec<T>.
/// T is the type returned by your callback method.
pub fn parse_file_by_lines_block_with_blank_lines_separator<P, T, C>(filename: P, lines_per_block: usize, separator_lines_between_block: usize, callback: fn(Vec<String>) -> T) -> Result<C, IOError>
where P: AsRef<Path>, C: FromIterator<T> {
    let fd = File::open(filename)?;
    let lines = BufReader::new(fd).lines();

    let mut items = Vec::new();
    let mut block_lines = Vec::new();
    let mut lines_to_ignore = 0;

    for line in lines {
        if lines_to_ignore > 0 {
            lines_to_ignore -= 1;
            continue;
        }

        block_lines.push(line?);

        if block_lines.len() == lines_per_block {
            items.push(callback(block_lines));
            block_lines = Vec::new();
            lines_to_ignore = separator_lines_between_block;
        }
    }

    Ok(items.into_iter().collect())
}

/// Attempts to open and parse a file containing digits into a vector of u32 vectors
///
/// For example, a file `digits.dat` with:
///     123455
///     891245
///
/// ```
/// use enontekio::parser;
///
/// let expected_digits = vec![vec![1 as u32, 2, 3, 4, 5, 5], vec![8, 9, 1, 2, 4, 5]];
/// let actual_digits = parser::parse_digits_grid_file("tests/parser/digits.dat").unwrap();
/// assert_eq!(actual_digits, expected_digits);
/// ```
pub fn parse_digits_grid_file<P>(filename: P) -> Result<Vec<Vec<u32>>, IOError> where P: AsRef<Path> {
    parse_file_by_line(filename, |line| parse_digits_grid_line(&line.unwrap()).unwrap())
}

/// Parses a string into a vector of digits
///
/// ```
/// use enontekio::parser;
///
/// let expected_digits = vec![1 as u32, 2, 3, 4, 5];
/// let actual_digits = parser::parse_digits_grid_line("12345").unwrap();
/// assert_eq!(actual_digits, expected_digits);
/// ```
pub fn parse_digits_grid_line(line: &str) -> Option<Vec<u32>> {
    line
        .chars()
        .map(|c| c.to_digit(10))
        .collect()
}

/// Attempts to open and parse a file containing characters into a vector of char vectors
///
/// For example, a file `chars.dat` with:
///     abcde
///     xizzy
///
/// ```
/// use enontekio::parser;
///
/// let expected_chars = vec![
///     vec!['a', 'b', 'c', 'd', 'e'],
///     vec!['x', 'i', 'z', 'z', 'y'],
/// ];
/// let actual_chars = parser::parse_chars_grid_file("tests/parser/chars.dat").unwrap();
/// assert_eq!(expected_chars, actual_chars);
/// ```
pub fn parse_chars_grid_file<P>(filename: P) -> Result<Vec<Vec<char>>, IOError> where P: AsRef<Path> {
    parse_file_by_line(filename, |line| parse_chars_grid_line(&line.unwrap()))
}

/// Parses a string into a vector of digits
///
/// ```
/// use enontekio::parser;
///
/// let expected_chars = vec!['a', 'b', 'c', 'd', 'e'];
/// let actual_chars = parser::parse_chars_grid_line("abcde");
/// assert_eq!(expected_chars, actual_chars);
/// ```
pub fn parse_chars_grid_line(line: &str) -> Vec<char> {
    line
        .chars()
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_file_by_line() {
        let expected = vec!['@', 'A', 'B', 'C'];
        let actual: Vec<_> = parse_file_by_line(
            "tests/parser/ascii.txt",
            |line| line.unwrap().parse::<u8>().unwrap() as char
        ).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_file_by_line_with_non_existing_file() {
        let result: Result<Vec<_>, _> = parse_file_by_line("/nonexisting", |_| ());

        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_parse_digits_grid_file() {
        let expected_digits = vec![vec![1 as u32, 2, 3, 4, 5, 5], vec![8, 9, 1, 2, 4, 5]];
        let actual_digits = parse_digits_grid_file("tests/parser/digits.dat").unwrap();

        assert_eq!(actual_digits, expected_digits);
    }

    #[test]
    fn test_parse_digits_grid_line() {
        assert_eq!(vec![1 as u32, 2, 3, 4, 5], parse_digits_grid_line("12345").unwrap());
    }

    #[test]
    fn parse_digits_grid_line_when_it_is_not() {
        assert_eq!(None, parse_digits_grid_line("This is not a digits line."));
    }
}
