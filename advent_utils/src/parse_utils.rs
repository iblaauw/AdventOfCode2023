use std::str::FromStr;

use super::generic_error::GenericError;

pub fn parse_value_list<T>(string: &str, separator: char) -> Result<Vec<T>, GenericError>
    where T : FromStr,
        GenericError : From<T::Err>
{
    let result_vec = string.split(separator)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<T>())
        .collect::<Result<Vec<T>, <T as FromStr>::Err>>()
        ?;

    Ok(result_vec)
}

pub fn expect_prefix<'a>(full_str: &'a str, expected_str: &str)
    -> Result<&'a str, GenericError>
{
    full_str.strip_prefix(expected_str)
        .ok_or_else(|| GenericError::new(format!("Expected prefix {}", expected_str)))
}