use crate::util::get_lines;

#[allow(dead_code)]
pub fn solve() {
  let lines = get_lines("./src/day02/problem2/sample.txt");

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
