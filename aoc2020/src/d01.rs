// https://adventofcode.com/2020/day/1

use std::fs;
use itertools::Itertools;

#[allow(dead_code)]
fn main_d01() -> Result<(i64, i64), String> {
  let target = 2020;
  fs::read_to_string("inputs/d01.txt")
    .map_err(|error| error.to_string())
    .and_then(|string| {
      let (numbers, errors) = parse_vectors(string, "\n");

      if !errors.is_empty() {
        dbg!(errors);
      }

      Option::zip(
        find_2_number(numbers.clone(), target).map(|(a, b)| a * b),
        find_3_numbers(numbers, target).map(|(a, b, c)| a * b * c),
      )
      .ok_or("unable to find solutions".to_string())
    })
}

fn find_2_number(numbers: Vec<i64>, target: i64) -> Option<(i64, i64)> {
  numbers.into_iter().tuple_combinations().find(|(a, b)| a + b == target)
}

fn find_3_numbers(numbers: Vec<i64>, target: i64) -> Option<(i64, i64, i64)> {
  numbers
    .into_iter()
    .tuple_combinations()
    .find(|(a, b, c)| a + b + c == target)
}

fn parse_vectors(input: String, separator: &str) -> (Vec<i64>, Vec<String>) {
  let (numbers, errors): (Vec<_>, Vec<_>) = input
    .split(separator)
    .map(|number| number.parse::<i64>().map_err(|_| number.to_string()))
    .partition(Result::is_ok);

  let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
  let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

  (numbers, errors)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse_vector_works() {
    let input = "1721, 979, 366, 299, 675, 1456".to_string();
    let expected = vec![1721, 979, 366, 299, 675, 1456];
    let (numbers, errors) = parse_vectors(input, ", ");

    assert_eq!(errors.len(), 0);
    assert_eq!(numbers, expected);
  }

  #[test]
  fn parse_vector_with_error_works() {
    let input = "1721, 979, 366, 299, cat".to_string();
    let expected = vec![1721, 979, 366, 299];
    let (numbers, errors) = parse_vectors(input, ", ");

    assert_eq!(errors.len(), 1);
    assert_eq!(numbers, expected);
  }

  #[test]
  fn find_2_number_works() {
    let numbers = vec![1721, 979, 366, 299, 675, 1456];
    let target = 2020;
    let expected = 514579;
    let result = find_2_number(numbers, target);

    assert_eq!(result.map(|a| a.0 * a.1), Some(expected));
  }

  #[test]
  fn find_3_number_works() {
    let numbers = vec![1721, 979, 366, 299, 675, 1456];
    let target = 2020;
    let expected = 241861950;
    let result = find_3_numbers(numbers, target);

    assert_eq!(result.map(|a| a.0 * a.1 * a.2), Some(expected));
  }

  #[test]
  fn main_works() {
    let expected = (1015476, 200878544);
    let result = main_d01();
    assert_eq!(result, Ok(expected));
  }
}
