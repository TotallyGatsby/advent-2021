use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[allow(dead_code)]
pub fn solve() {
  let path = Path::new("./src/day02/problem2/input.txt");

  let display = path.display();

  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
  };

  let file_reader = BufReader::new(file);

  let lines = file_reader.lines();
  let mut horizontal = 0;
  let mut depth = 0;
  let mut aim = 0;
  for line in lines {
    if let Ok(instruction) = line {
      let split = instruction.split(' ');

      let words: Vec<&str> = split.collect();
      let dir = words[0];
      let amount: i32 = words[1].parse().unwrap();

      if dir == "forward" {
        horizontal += amount;
        depth += aim * amount;
      } else if dir == "down" {
        aim += amount;
      } else if dir == "up" {
        aim -= amount;
      }
    }
  }
  println!("Horizontal: {} Depth: {}", horizontal, depth);
  println!("Result: {}", horizontal * depth)
}
