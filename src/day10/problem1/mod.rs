use std::collections::VecDeque;
#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let mut corrupt_lines = Vec::new();

  let mut corruption_score = 0;

  for line in lines {
    let mut char_stack = VecDeque::new();

    let mut is_corrupt = false;
    for char in line.chars() {
      if char == '<' || char == '(' || char == '{' || char == '[' {
        char_stack.push_back(char);
      }

      if char == '>' {
        if char_stack[char_stack.len() - 1] == '<' {
          char_stack.pop_back();
        } else {
          is_corrupt = true;
          corruption_score += 25137;
          break;
        }
      } else if char == ']' {
        if char_stack[char_stack.len() - 1] == '[' {
          char_stack.pop_back();
        } else {
          is_corrupt = true;
          corruption_score += 57;
          break;
        }
      } else if char == '}' {
        if char_stack[char_stack.len() - 1] == '{' {
          char_stack.pop_back();
        } else {
          is_corrupt = true;
          corruption_score += 1197;
          break;
        }
      } else if char == ')' {
        if char_stack[char_stack.len() - 1] == '(' {
          char_stack.pop_back();
        } else {
          is_corrupt = true;
          corruption_score += 3;
          break;
        }
      }
    }

    if is_corrupt {
      corrupt_lines.push(line);
    }
  }

  //println!("Corrupt Lines: {:?}", corrupt_lines);
  println!("Score: {}", corruption_score);
}
