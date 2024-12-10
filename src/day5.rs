use std::collections::HashMap;
use std::collections::HashSet;

#[aoc(day5, part1, Chars)]
pub fn part1(input: &str) -> i32 {
  let mut total = 0i32;
  let rule_and_print_orders = parse_input(input);

  for print_order in rule_and_print_orders.print_orders {
    if is_print_order_correct(&print_order.original_order, &rule_and_print_orders.rules) {
      total += print_order.get_middle_value();
    }
  }

  total
}

#[aoc(day5, part2, Chars)]
pub fn part2(input: &str) -> i32 {
  let mut total = 0i32;
  let rule_and_print_orders = parse_input(input);

  for print_order in rule_and_print_orders.print_orders {
    if !is_print_order_correct(&print_order.original_order, &rule_and_print_orders.rules) {
      let corrected_print_order = build_correct_order_from_rules(&rule_and_print_orders.rules, &print_order);
      total += corrected_print_order.get_middle_value();
    }
  }

  total
}

fn build_correct_order_from_rules(rules: &HashMap<i32, Rule>, print_order: &PrintOrder) -> PrintOrder {
  let mut corrected_order: Vec<i32> = print_order.original_order.clone();

  while !is_print_order_correct(&corrected_order, rules) {
    let swapped = false;
    for i in 0..corrected_order.len() {
      if swapped {
        break;
      }
      let page_number = corrected_order[i];
      for j in (i + 1)..corrected_order.len() {
        if swapped {
          break;
        }
        let page_number_after = corrected_order[j];
        if let Some(rule) = rules.get(&page_number) {
          if rule.pages_before.contains(&page_number_after) {
            corrected_order.swap(i, j);
          }
        }
      }
    }
  }

  PrintOrder {
    original_order: corrected_order
  }
}

fn is_print_order_correct(print_order: &Vec<i32>, rules: &HashMap<i32, Rule>) -> bool {
  let print_order_pages_set: HashSet<i32> = HashSet::from_iter(print_order.iter().cloned());
  let mut visited_page_numbers: HashSet<i32> = HashSet::new();
  for page_number in print_order {
    if let Some(rule) = rules.get(&page_number) {
      for page_number_after in &rule.pages_after {
        if visited_page_numbers.contains(page_number_after) {
          // Exit early if the page is supposed to be after but appears before
          return false;
        }
      }

      for page_number_before in &rule.pages_before {
        if !visited_page_numbers.contains(page_number_before) && print_order_pages_set.contains(page_number_before) {
          // Exit early if the page is supposed to appear before but appears after
          return false;
        }
      }
    };
    visited_page_numbers.insert(page_number.clone());
  }

  true
}

struct RulesAndPrintOrders {
  rules: HashMap<i32, Rule>,
  print_orders: Vec<PrintOrder>,
}

struct Rule {
  pages_before: Vec<i32>,
  pages_after: Vec<i32>,
}

struct PrintOrder {
  original_order: Vec<i32>,
}

trait GetMiddleValue {
  fn get_middle_value(&self) -> i32;
}

impl GetMiddleValue for PrintOrder {
  fn get_middle_value(&self) -> i32 {
    let length = self.original_order.len();
    if length % 2 == 0 {
      // To be clear we should NEVER panic in production
      panic!("Print order length is even");
    }

    let middle = length / 2;
    self.original_order[middle]
  }
}

fn parse_input(input: &str) -> RulesAndPrintOrders {
  let mut parsing_rules = true;
  let mut rules: HashMap<i32, Rule> = HashMap::new();
  let mut print_orders: Vec<PrintOrder> = Vec::new();
  for line in input.lines() {
    if line.is_empty() {
      parsing_rules = false;
      continue;
    }

    if parsing_rules {
      let (page_before, page_after) = parse_rule(line);
      if let Some(current_entry) = rules.get_mut(&page_before) {
        current_entry.pages_after.push(page_after);
      } else {
        rules.insert(page_before, Rule {
          pages_before: Vec::new(),
          pages_after: vec![page_after],
        });
      }

      if let Some(current_entry) = rules.get_mut(&page_after) {
        current_entry.pages_before.push(page_before);
      } else {
        rules.insert(page_after, Rule {
          pages_before: vec![page_before],
          pages_after: Vec::new(),
        });
      }
    } else {
      print_orders.push(parse_print_order(line));
    }
  }

  RulesAndPrintOrders {
    print_orders,
    rules,
  }
}

fn parse_rule(line: &str) -> (i32, i32) {
  let mut parts = line.split("|");
  let page_before = parts.next().unwrap().parse::<i32>().unwrap();
  let page_after = parts.next().unwrap().parse::<i32>().unwrap();

  (
    page_before,
    page_after,
  )
}

fn parse_print_order(line: &str) -> PrintOrder {
  let parts = line
    .split(",")
    .map(|s| s.parse::<i32>().unwrap())
    .collect();


  PrintOrder {
    original_order: parts,
  }
}

#[cfg(test)]
mod parse_print_order {
  use super::*;

  #[test]
  fn parses_print_order() {
    let input = "1,2,3,4,5,6,7,8,9,10";

    let print_order = parse_print_order(input);

    assert_eq!(print_order.original_order, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  }
}

#[cfg(test)]
mod parse_rule {
  use super::*;

  #[test]
  fn parses_rule() {
    let input = "11|12";

    let (page_before, page_after) = parse_rule(input);

    assert_eq!(page_before, 11);
    assert_eq!(page_after, 12);
  }
}

#[cfg(test)]
mod parse_input {
  use super::*;

  #[test]
  fn parses_input() {
    let input = "11|12\n11|13\n13|12\n\n1,2,3,4,5,6,7,8,9,10\n11,12,13,14,15,16,17,18,19,20";

    let rules_and_print_orders = parse_input(input);

    assert_eq!(rules_and_print_orders.print_orders.len(), 2);
    assert_eq!(rules_and_print_orders.print_orders[0].original_order, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(rules_and_print_orders.print_orders[1].original_order, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    assert_eq!(rules_and_print_orders.rules.len(), 3);
    assert_eq!(rules_and_print_orders.rules.get(&11).unwrap().pages_before, vec![]);
    assert_eq!(rules_and_print_orders.rules.get(&11).unwrap().pages_after, vec![12, 13]);
    assert_eq!(rules_and_print_orders.rules.get(&12).unwrap().pages_before, vec![11, 13]);
    assert_eq!(rules_and_print_orders.rules.get(&12).unwrap().pages_after, vec![]);
    assert_eq!(rules_and_print_orders.rules.get(&13).unwrap().pages_before, vec![11]);
    assert_eq!(rules_and_print_orders.rules.get(&13).unwrap().pages_after, vec![12]);
  }
}

#[cfg(test)]
mod print_order_tests {
  use super::*;

  #[test]
  fn get_middle_value() {
    let print_order = PrintOrder {
      original_order: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    };

    assert_eq!(print_order.get_middle_value(), 5);
  }

  #[test]
  #[should_panic]
  fn get_middle_value_even() {
    let print_order = PrintOrder {
      original_order: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };

    print_order.get_middle_value();
  }
}

#[cfg(test)]
mod part1_tests {
  use super::*;

  #[test]
  fn part1_test() {
    let input = "1|2
1|2
2|3
2|4
3|4

1,2,3,4,5
5,4,3,2,1
1,3,4
2,3,4
3,4,5";

    let result = part1(input);

    assert_eq!(result, 3 + 3 + 3 + 4);
  }

  #[test]
  fn part1_provided_proof() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let result = part1(input);

    assert_eq!(result, 143);
  }
}

#[cfg(test)]
mod build_correct_order_from_rules_tests {
  use super::*;

  #[test]
  fn build_correct_order_from_rules_test() {
    let rules = HashMap::from([
      (1, Rule {
        pages_before: vec![],
        pages_after: vec![2],
      }),
      (2, Rule {
        pages_before: vec![1],
        pages_after: vec![3],
      }),
      (3, Rule {
        pages_before: vec![2],
        pages_after: vec![4],
      }),
      (4, Rule {
        pages_before: vec![3],
        pages_after: vec![5],
      }),
      (5, Rule {
        pages_before: vec![4],
        pages_after: vec![],
      }),
    ]);
    let print_order = PrintOrder {
      original_order: vec![5, 3, 4, 1, 2],
    };

    assert_eq!(build_correct_order_from_rules(&rules, &print_order).original_order, vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn from_real_data_set() {
    let rules = HashMap::from([
      (37, Rule {
        pages_before: vec![47, 65],
        pages_after: vec![75, 61, 87],
      }),
      (75, Rule {
        pages_before: vec![37, 47, 65],
        pages_after: vec![31, 61, 87],
      }),
      (61, Rule {
        pages_before: vec![37, 65, 47, 75],
        pages_after: vec![31, 87],
      }),
      (65, Rule {
        pages_before: vec![],
        pages_after: vec![31, 37, 47, 61, 75, 87],
      }),
      (31, Rule {
        pages_before: vec![47, 61, 65, 75, 87],
        pages_after: vec![],
      }),
      (47, Rule {
        pages_before: vec![65],
        pages_after: vec![31, 37, 61, 75, 87],
      }),
      (87, Rule {
        pages_before: vec![37, 47, 57, 61, 65, 75],
        pages_after: vec![31],
      }),
    ]);

    let print_order = PrintOrder {
      original_order: vec![37, 75, 61, 65, 31, 47, 87],
    };

    assert_eq!(build_correct_order_from_rules(&rules, &print_order).original_order, vec![65, 47, 37, 75, 61, 87, 31]);
  }
}
