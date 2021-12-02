use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn solve() {
  let path = Path::new("./src/day01/problem1/input.txt");

  let display = path.display();

  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
  };

  let file_reader = BufReader::new(file);

  let lines = file_reader.lines();

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
