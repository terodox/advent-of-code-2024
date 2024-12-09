/*
"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:


..X...
.SAMX.
.A..A.
XMAS.S
.X....
The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
Take a look at the little Elf's word search. How many times does XMAS appear?
*/
use regex::Regex;

#[aoc(day4, part1, Chars)]
pub fn part1(input: &str) -> i32 {
  // vertical line search (forward and backwards)
  // diagonal line search (forward and backwards)
  let mut total = 0;
  let xmas_finder = Regex::new(r"XMAS").unwrap();
  let samx_finder = Regex::new(r"SAMX").unwrap();

  // horizontal line search (forward and backwards)
  total += search_horizontal(input.lines().collect(), &xmas_finder, &samx_finder);
  total += search_vertical(input.lines().collect(), &xmas_finder, &samx_finder);
  total += search_diagonal(input.lines().collect(), &xmas_finder, &samx_finder);

  total
}

#[aoc(day4, part2, Chars)]
pub fn part2(str_input: &str) -> i32 {
  let mut total = 0;
  // Convert input to Vec<String>
  let input = str_input.lines().map(|line| line.to_string()).collect::<Vec<String>>();


  let indexable_input = IndexableInput { input };

  for y in 0..str_input.lines().count() {
    let line = str_input.lines().nth(y).unwrap();
    if y + 2 > str_input.lines().count() - 1 {
      break;
    }
    for x in 0..line.len() {
      if x + 2 > line.len() - 1 {
        break;
      }

      let char = indexable_input.get(x, y);
      /*
      M-M
      -A-
      S-S
      */
      if char == 'M'
        && indexable_input.get(x + 2, y) == 'M'
        && indexable_input.get(x + 1, y + 1) == 'A'
        && indexable_input.get(x, y + 2) == 'S'
        && indexable_input.get(x + 2, y + 2) == 'S' {
          total += 1;
          continue;
      }

      /*
      S-S
      -A-
      M-M
      */
      if char == 'S'
        && indexable_input.get(x + 2, y) == 'S'
        && indexable_input.get(x + 1, y + 1) == 'A'
        && indexable_input.get(x, y + 2) == 'M'
        && indexable_input.get(x + 2, y + 2) == 'M' {
          total += 1;
          continue;
      }

      /*
      M-S
      -A-
      M-S
      */
      if char == 'M'
        && indexable_input.get(x + 2, y) == 'S'
        && indexable_input.get(x + 1, y + 1) == 'A'
        && indexable_input.get(x, y + 2) == 'M'
        && indexable_input.get(x + 2, y + 2) == 'S' {
          total += 1;
          continue;
      }

      /*
      S-M
      -A-
      S-M
      */
      if char == 'S'
        && indexable_input.get(x + 2, y) == 'M'
        && indexable_input.get(x + 1, y + 1) == 'A'
        && indexable_input.get(x, y + 2) == 'S'
        && indexable_input.get(x + 2, y + 2) == 'M' {
          total += 1;
          continue;
      }
    }
  }

  total
}

struct IndexableInput {
  input: Vec<String>
}

trait Indexable {
  fn get(&self, x: usize, y: usize) -> char;
}

impl Indexable for IndexableInput {
  fn get(&self, x: usize, y: usize) -> char {
      self.input[y].chars().nth(x).unwrap()
  }
}

fn search_horizontal(input: Vec<&str>, xmas_finder: &Regex, samx_finder: &Regex) -> i32 {
  let mut total = 0;

  for line in input {
    total += xmas_finder.captures_iter(line).count();
    total += samx_finder.captures_iter(line).count();
  }

  // I know this is dangerous
  i32::try_from(total).unwrap()
}

fn search_vertical(input: Vec<&str>, xmas_finder: &Regex, samx_finder: &Regex) -> i32 {
  let mut total = 0;

  for i in 0..input[0].len() {
    let mut line = String::new();
    for j in 0..input.len() {
      line.push(input[j].chars().nth(i).unwrap());
    }
    total += xmas_finder.captures_iter(&line).count();
    total += samx_finder.captures_iter(&line).count();
  }

  i32::try_from(total).unwrap()
}

fn search_diagonal(input: Vec<&str>, xmas_finder: &Regex, samx_finder: &Regex) -> i32 {
  let mut total = 0;
  let dimension_length = input.len();

  // Assumptions that all lines are of equal length
  // Conveniently the input is a square
  let mut lines_to_search: Vec<String> = vec![];
  // Top Left
  for x in 0..dimension_length {
    let mut line = String::new();
    for y in 0..dimension_length {
      if x < y {
        break;
      }
      line.push(input[x - y].chars().nth(y).unwrap());
    }
    if line.len() < 4 {
      continue;
    }
    lines_to_search.push(line);
  }

  // Top Right
  for y in (0..dimension_length).rev() {
    let mut line = String::new();
    for x in 0..dimension_length {
      if x + y > dimension_length - 1 {
        break;
      }
      line.push(input[x].chars().nth(y + x).unwrap());
    }
    if line.len() < 4 {
      continue;
    }
    lines_to_search.push(line);
  }

  // Bottom Left
  for x in (1..dimension_length).rev() {
    let mut line = String::new();
    for y in 0..dimension_length {
      if x + y > dimension_length - 1 {
        break;
      }
      line.push(input[x + y].chars().nth(y).unwrap());
    }
    if line.len() < 4 {
      continue;
    }
    lines_to_search.push(line);
  }

  // Bottom Right
  for x in (1..dimension_length).rev() {
    let mut line = String::new();
    for y in (0..dimension_length).rev() {
      if y < x {
        break;
      }
      let factor = dimension_length - 1 - y;

      line.push(input[x + factor].chars().nth(y).unwrap());
    }
    if line.len() < 4 {
      continue;
    }
    lines_to_search.push(line);
  }

  for line_to_search in lines_to_search {
    total += xmas_finder.captures_iter(&line_to_search).count();
    total += samx_finder.captures_iter(&line_to_search).count();
  }

  i32::try_from(total).unwrap()
}

#[cfg(test)]
mod day4_part1 {
  use super::*;

  #[test]
  fn overlapping_matches_should_count_twice() {
    let xmas_finder = Regex::new(r"XMAS").unwrap();
    let samx_finder = Regex::new(r"SAMX").unwrap();
    assert_eq!(
      xmas_finder.captures_iter("XMASAMX").count() +
      samx_finder.captures_iter("XMASAMX").count(),
      2
    );
  }

  #[test]
  fn provided_test_case_should_pass() {
    assert_eq!(
      part1(
        "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
"
      ),
      18
    );
  }
}

#[cfg(test)]
mod search_horizontal_tests {
  use super::*;

  #[test]
  fn horizontal_search_finds_all_matches() {
    let input = vec![
      "AXMASASAMXX",
      "XSAMXSXMASA",
      "XMASSSSAMXM",
      "XXXMASXMASM",
    ];
    assert_eq!(search_horizontal(input, &Regex::new(r"XMAS").unwrap(), &Regex::new(r"SAMX").unwrap()), 8);
  }
}

#[cfg(test)]
mod search_vertical_tests {
  use super::*;

  #[test]
  fn vertical_search_finds_all_matches() {
    let input = vec![
      "MM",
      "XS",
      "MA",
      "AM",
      "SX",
      "SX",
      "AM",
      "MA",
      "XS",
      "XX",
    ];
    assert_eq!(search_vertical(input, &Regex::new(r"XMAS").unwrap(), &Regex::new(r"SAMX").unwrap()), 4);
  }
}

#[cfg(test)]
mod search_diagonal_tests {
  use super::*;

  #[test]
  fn diagonal_search_finds_all_matches() {
    let input = vec![
      "XXXX",
      "MMMM",
      "AAAA",
      "SSSS",
    ];
    assert_eq!(search_diagonal(input, &Regex::new(r"XMAS").unwrap(), &Regex::new(r"SAMX").unwrap()), 2);
  }

  #[test]
  fn diagonal_search_finds_all_reversed_matches() {
    let input = vec![
      "---SS---",
      "--A--A-X",
      "-M----M-",
      "X----A-X",
      "S---S---",
      "-A-A----",
      "--M-----",
      "-X-X----",
    ];
    assert_eq!(search_diagonal(input, &Regex::new(r"XMAS").unwrap(), &Regex::new(r"SAMX").unwrap()), 5);
  }
}

#[cfg(test)]
mod part2 {
  use super::*;

  #[test]
  fn provided_test_case_should_pass() {
    assert_eq!(
      part2(
        ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
"
      ),
      9
    );
  }
}