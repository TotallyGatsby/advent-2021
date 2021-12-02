use crate::util::get_lines;
use std::collections::VecDeque;

#[allow(dead_code)]
pub fn solve() {
  let lines = get_lines("./src/day01/problem2/input.txt");

  let mut prev_depths = VecDeque::new();
  let mut previous_window = 0;
  let mut depth_increases = 0;
  for line in lines {
    if let Ok(depth) = line {
      let current_depth: i32 = depth.parse().unwrap();

      if prev_depths.len() < 3 {
        prev_depths.push_back(current_depth);
        previous_window += current_depth;
      } else {
        let mut current_window = previous_window;
        let old_depth = prev_depths.pop_front();
        prev_depths.push_back(current_depth);
        current_window -= old_depth.unwrap();
        current_window += current_depth;
        if current_window > previous_window {
          depth_increases += 1;
        }
        previous_window = current_window;
      }
    }
  }

  println!("Total Depth Increases: {}", depth_increases);
}
