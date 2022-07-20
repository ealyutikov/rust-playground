use std::{collections::HashSet, fs};

fn main_d01() -> Result<i64, String> {
  let target = 2020;
  fs::read_to_string("inputs/d01.txt")
    .map_err(|error| error.to_string())
    .and_then(|string| {
      let (numbers, errors) = parse_vectors(string, "\n");

      if !errors.is_empty() {
        dbg!(errors);
      }

      find_solution(numbers, target)
    })
}

fn find_solution(numbers: Vec<i64>, target: i64) -> Result<i64, String> {
  let set: HashSet<i64> = HashSet::from_iter(numbers.iter().cloned());

  let result: Vec<(i64, &i64)> = numbers
    .into_iter()
    .flat_map(|n| {
      let y = target - n;
      set.get(&y).map(|f| (n, f))
    })
    .collect();

  match result[..] {
    [(num1, num2), _] => Ok(num1 * num2),
    [_, _, ..] => Err(format!("more than one solution found: {:?}", result).to_string()),
    _ => Err("unable to find solution".to_string()),
  }
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
  fn find_solution_works() {
    let numbers = vec![1721, 979, 366, 299, 675, 1456];
    let target = 2020;
    let expected = 514579;
    let result = find_solution(numbers, target);

    assert_eq!(result, Ok(expected));
  }

  #[test]
  fn main_works() {
    let result = main_d01();
    assert_eq!(result, Ok(1015476));
  }
}
