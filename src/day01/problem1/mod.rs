use crate::util::get_lines;

#[allow(dead_code)]
pub fn solve() {
  let lines = get_lines(input_path: &str)("./src/day01/problem/input.txt");

  let mut previous_depth = -1;
  let mut depth_increases = 0;
  for line in lines {
    if let Ok(depth) = line {
      let current_depth: i32 = depth.parse().unwrap();
      /*
      println!(
        "Current Depth: {}, Previous Depth: {}",
        current_depth, previous_depth
      );*/
      if previous_depth != -1 && current_depth > previous_depth {
        depth_increases += 1;
      }
      previous_depth = current_depth;
    }
  }

  println!("Total Depth Increases: {}", depth_increases);
}
