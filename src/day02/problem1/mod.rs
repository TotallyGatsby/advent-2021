use crate::util::get_lines;

#[allow(dead_code)]
pub fn solve() {
  let lines = get_lines("./src/day02/problem1/input.txt");

  let mut horizontal = 0;
  let mut depth = 0;
  for line in lines {
    if let Ok(instruction) = line {
      let split = instruction.split(' ');

      let words: Vec<&str> = split.collect();
      let dir = words[0];
      let amount: i32 = words[1].parse().unwrap();

      if dir == "forward" {
        horizontal += amount;
      } else if dir == "down" {
        depth += amount;
      } else if dir == "up" {
        depth -= amount;
      }
    }
  }
  println!("Horizontal: {} Depth: {}", horizontal, depth);
  println!("Result: {}", horizontal * depth)
}
