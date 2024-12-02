/*
The Chief Historian is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last anyone heard, he was visiting locations that are historically significant to the North Pole; a group of Senior Historians has asked you to accompany them as they check the places they think he was most likely to visit.

As each location is checked, they will mark it on their list with a star. They figure the Chief Historian must be in one of the first fifty places they'll look, so in order to save Christmas, you need to help them get fifty stars on their list before Santa takes off on December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You haven't even left yet and the group of Elvish Senior Historians has already hit a problem: their list of locations to check is currently empty. Eventually, someone decides that the best place to check first would be the Chief Historian's office.

Upon pouring into the office, everyone confirms that the Chief Historian is indeed nowhere to be found. Instead, the Elves discover an assortment of notes and lists of historically significant locations! This seems to be the planning the Chief Historian was doing before he left. Perhaps these notes can be used to determine which locations to search?

Throughout the Chief's office, the historically significant locations are listed not by name but by a unique number called the location ID. To make sure they don't miss anything, The Historians split into two groups, each searching the office and trying to create their own complete list of location IDs.

There's just one problem: by holding the two lists up side by side (your puzzle input), it quickly becomes clear that the lists aren't very similar. Maybe you can help The Historians reconcile their lists?

For example:

3   4
4   3
2   5
1   3
3   9
3   3
Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are. Pair up the smallest number in the left list with the smallest number in the right list, then the second-smallest left number with the second-smallest right number, and so on.

Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances. For example, if you pair up a 3 from the left list with a 7 from the right list, the distance apart is 4; if you pair up a 9 with a 3, the distance apart is 6.

In the example list above, the pairs and distances would be as follows:

The smallest number in the left list is 1, and the smallest number in the right list is 3. The distance between them is 2.
The second-smallest number in the left list is 2, and the second-smallest number in the right list is another 3. The distance between them is 1.
The third-smallest number in both lists is 3, so the distance between them is 0.
The next numbers to pair up are 3 and 4, a distance of 1.
The fifth-smallest numbers in each list are 3 and 5, a distance of 2.
Finally, the largest number in the left list is 4, while the largest number in the right list is 9; these are a distance 5 apart.
To find the total distance between the left list and the right list, add up the distances between all of the pairs you found. In the example above, this is 2 + 1 + 0 + 1 + 2 + 5, a total distance of 11!

Your actual left and right lists contain many location IDs. What is the total distance between your lists?
*/

use std::collections::HashMap;

#[aoc(day1, part1, Chars)]
pub fn part1(input: &str) -> i32 {
  let (left_sorted_number_arr, right_sorted_number_arr) = get_sorted_arrays(input);

  let mut difference_sum = 0;
  for (pos, left_item) in left_sorted_number_arr.iter().enumerate() {
    let right_item = right_sorted_number_arr[pos];
    difference_sum += (left_item - right_item).abs();
  }

  difference_sum
}

#[aoc(day1, part2, Chars)]
pub fn part2(input: &str) -> i32 {
  let (left_sorted_number_arr, right_sorted_number_arr) = get_sorted_arrays(input);

  let mut right_number_occurrence_count: HashMap<i32, i32> = HashMap::new();
  for right_item in right_sorted_number_arr.iter() {
    let count = right_number_occurrence_count.entry(*right_item).or_insert(0);
    *count += 1;
  }

  let mut similarity_score = 0;
  for (_, left_item) in left_sorted_number_arr.iter().enumerate() {
    similarity_score += left_item * right_number_occurrence_count.get(left_item).unwrap_or(&0i32);
  }

  similarity_score
}

fn get_sorted_arrays(input: &str) -> (Vec<i32>, Vec<i32>) {
  let lines = input.lines();
  let mut left_arr: Vec<&str> = Vec::new();
  let mut right_arr: Vec<&str> = Vec::new();
  for line in lines {
    let one_line_split = line.split("   ");
    let collected_items = one_line_split.collect::<Vec<&str>>();
    left_arr.push(collected_items[0]);
    right_arr.push(collected_items[1]);
  }

  let mut left_sorted_number_arr = left_arr.iter().map(|item| item.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  left_sorted_number_arr.sort();
  let mut right_sorted_number_arr: Vec<i32> = right_arr.iter().map(|item| item.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  right_sorted_number_arr.sort();

  (left_sorted_number_arr, right_sorted_number_arr)
}