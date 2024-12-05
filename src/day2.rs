/*
--- Day 2: Red-Nosed Reports ---
Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.

While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

The levels are either all increasing or all decreasing.
Any two adjacent levels differ by at least one and at most three.
In the example above, the reports can be found safe or unsafe by checking those rules:

7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?
*/

#[aoc(day2, part1, Chars)]
pub fn part1(input: &str) -> i32 {
  let reports = input.lines().map(|line| parse_report(line));

  // reports.for_each(|report| println!("{:?}", report));

  reports.filter(|report| report.is_safe()).count() as i32
}

/*
The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

7 6 4 2 1: Safe without removing any level.
1 2 7 8 9: Unsafe regardless of which level is removed.
9 7 6 2 1: Unsafe regardless of which level is removed.
1 3 2 4 5: Safe by removing the second level, 3.
8 6 4 4 1: Safe by removing the third level, 4.
1 3 6 7 9: Safe without removing any level.
Thanks to the Problem Dampener, 4 reports are actually safe!

Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?
*/

#[aoc(day2, part2, Chars)]
pub fn part2(input: &str) -> i32 {
  let reports = input.lines().map(|line| parse_report(line));

  reports.filter(|report| report.is_safe_part_2()).count() as i32
  // for report in reports.filter(|report| !report.is_safe() && report.is_safe_part_2()).enumerate() {
  //   println!("{:?}", report);
  // }
  // 12
}

fn parse_report(input: &str) -> Report {
  let mut levels = Vec::new();
  for level in input.split_whitespace() {
    levels.push(level.parse::<i32>().unwrap());
  }
  Report { levels }
}

trait IsSafe {
  fn is_safe(&self) -> bool;
}

trait IsSafePart2<T: IsSafe = Self> {
  fn is_safe_part_2(&self) -> bool;
}

#[derive(Debug)]
struct Report {
  levels: Vec<i32>,
}

impl IsSafe for Report {
  fn is_safe(&self) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    let mut previous_value = None;
    for level in self.levels.iter() {
      if previous_value.is_none() {
        previous_value = Some(level);
        continue;
      }

      if level > previous_value.unwrap() {
        increasing = true;
      }
      if level < previous_value.unwrap() {
        decreasing = true;
      }
      if (increasing && decreasing) || !is_level_diff_in_range(level, previous_value.unwrap()) {
        return false;
      }

      previous_value = Some(level);
    }

    true
  }
}

impl IsSafePart2 for Report {
  fn is_safe_part_2(&self) -> bool {
    for (i, _) in self.levels.iter().enumerate() {
      let mut levels = self.levels.clone();
      levels.remove(i);

      if (Report { levels }.is_safe()) {
        return true;
      }
    }

    false
  }
}

fn is_level_diff_in_range(level: &i32, previous_value: &i32) -> bool {
  let level_diff = (level - previous_value).abs();

  if level_diff >= 1 && level_diff <= 3 {
    return true;
  }

  false
}

#[cfg(test)]
mod tests_part_2 {
  use super::*;

  #[test]
  fn when_the_report_is_safe_then_the_report_is_safe_part_2() {
    let good_report = Report { levels: vec![1, 2, 3, 4] };
    assert!(good_report.is_safe_part_2());
  }

  #[test]
  fn when_the_report_is_not_safe_then_the_report_is_not_safe_part_2() {
    let bad_report = Report { levels: vec![1, 2, 7, 14, 19] };
    assert!(bad_report.is_safe_part_2() == false);
  }

  #[test]
  fn when_the_report_is_safe_but_one_level_is_removed_then_the_report_is_safe_part_2() {
    let bad_report = Report { levels: vec![1, 2, 3, 18, 4] };
    assert!(bad_report.is_safe_part_2());
  }

  #[test]
  fn when_the_report_is_safe_but_the_previous_level_is_removed_then_the_report_is_safe_part_2() {
    let bad_report = Report { levels: vec![18, 2, 3, 4, 5] };
    assert!(bad_report.is_safe_part_2());
  }

  #[test]
  fn when_the_report_is_safe_but_one_level_is_removed_descending_then_the_report_is_safe_part_2() {
    let bad_report = Report { levels: vec![4, 3, 2, 18, 1] };
    assert!(bad_report.is_safe_part_2());
  }

  #[test]
  fn when_the_report_is_safe_but_the_previous_level_is_removed_descending_then_the_report_is_safe_part_2() {
    let bad_report = Report { levels: vec![18, 5, 4, 3, 2] };
    assert!(bad_report.is_safe_part_2());
  }

  #[test]
  fn when_the_report_requires_multiple_removals_to_be_safe_then_the_report_is_not_safe_part_2() {
    let bad_report = Report { levels: vec![18, 5, 4, 3, 22, 2] };
    assert!(bad_report.is_safe_part_2() == false);
  }

  #[test]
  fn scratch() {
    let bad_report = Report { levels: vec![52, 47, 49, 46, 43, 41, 40] };
    assert!(bad_report.is_safe_part_2() == false);
  }
}

#[cfg(test)]
mod tests_part_1 {
  use super::*;

  #[test]
  fn when_there_is_a_change_in_levels_greater_than_three_then_the_report_is_not_safe() {
    let bad_report = Report { levels: vec![1, 2, 7, 8, 9] };
    assert!(bad_report.is_safe() == false);
  }

  #[test]
  fn when_there_is_a_change_in_levels_greater_less_than_one_then_the_report_is_not_safe() {
    let bad_report = Report { levels: vec![1, 2, 2, 3, 4] };
    assert!(bad_report.is_safe() == false);
  }

  #[test]
  fn when_there_is_a_mix_of_ascending_and_descending_changes_then_the_report_is_not_safe() {
    let bad_report = Report { levels: vec![1, 2, 1, 3, 4] };
    assert!(bad_report.is_safe() == false);
  }

  #[test]
  fn when_levels_are_only_ascending_then_the_report_is_safe() {
    let bad_report = Report { levels: vec![1, 2, 3, 4] };
    assert!(bad_report.is_safe());
  }

  #[test]
  fn when_levels_are_only_descending_then_the_report_is_safe() {
    let bad_report = Report { levels: vec![4, 3, 2, 1] };
    assert!(bad_report.is_safe());
  }
}