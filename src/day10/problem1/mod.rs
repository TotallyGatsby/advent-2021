use std::collections::VecDeque;
#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let mut incomplete_lines = Vec::new();

  let mut corruption_score = 0;
  let mut autocomplete_scores = Vec::new();

  for line in lines {
    let mut char_stack = VecDeque::new();

    // Build the stack of opening braces
    // pop if a matching close brace is found
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

    // Autocomplete
    if !is_corrupt {
      let mut cur_score: usize = 0;
      while let Some(bracket) = char_stack.pop_back() {
        cur_score *= 5;
        if bracket == '(' {
          cur_score += 1;
        } else if bracket == '[' {
          cur_score += 2;
        } else if bracket == '{' {
          cur_score += 3;
        } else if bracket == '<' {
          cur_score += 4;
        }
      }

      autocomplete_scores.push(cur_score);
    }
    if !is_corrupt {
      incomplete_lines.push(line);
    }
  }

  autocomplete_scores.sort_unstable();

  //println!("Corrupt Lines: {:?}", corrupt_lines);
  println!("Corruption Score: {}", corruption_score);
  println!("Autocomplete Scores: {:?}", autocomplete_scores);
  println!(
    "Central Autocomplete Score: {}",
    autocomplete_scores[autocomplete_scores.len() / 2]
  );
}
