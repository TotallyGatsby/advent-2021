use crate::util::get_file_reader;
use std::io::BufRead;

mod bingo;

#[allow(dead_code)]
pub fn solve() {
  let input = get_file_reader("./src/day04/problem1/input.txt");
  let lines: Vec<String> = input.lines().collect::<Result<_, _>>().unwrap();

  let called_nums = (&lines[0]).split(',').collect::<Vec<_>>();
  //println!("Called Numbers: {}", called_nums);

  let mut idx = 2;
  let mut cards = Vec::new();

  while idx < lines.len() {
    cards.push(bingo::Card::from_lines(&lines[idx..idx + 5]));
    idx += 6;
  }

  let mut bingo_val;
  let mut bingo = false;
  for num in called_nums {
    let val: u16 = num.parse().unwrap();
    println!("Calling: {}", val);

    for idx in 0..cards.len() {
      bingo = cards[idx].mark(val);
      if bingo {
        bingo_val = val;
        println!(
          "Card ID: {} Card Sum: {}, Final Value: {}, Answer: {}",
          idx,
          cards[idx].sum,
          bingo_val,
          cards[idx].sum * bingo_val
        );
        break;
      }
    }
    if bingo {
      break;
    }
  }
}
