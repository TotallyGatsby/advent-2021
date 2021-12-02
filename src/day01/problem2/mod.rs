use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[allow(dead_code)]
pub fn solve() {
  let path = Path::new("./src/day01/problem1/input.txt");

  let display = path.display();

  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
  };

  let file_reader = BufReader::new(file);

  let lines = file_reader.lines();

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
