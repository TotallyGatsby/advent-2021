use crate::util::get_file_reader;
use std::io::BufRead;

mod bingo;

pub fn solve() {
  let input = get_file_reader("./src/day04/problem2/input.txt");
  let lines: Vec<String> = input.lines().collect::<Result<_, _>>().unwrap();

  let called_nums = (&lines[0]).split(',').collect::<Vec<_>>();
  //println!("Called Numbers: {}", called_nums);

  let mut idx = 2;
  let mut cards = Vec::new();

  while idx < lines.len() {
    cards.push(bingo::Card::from_lines(&lines[idx..idx + 5]));
    idx += 6;
  }

  for num in called_nums {
    let val: u16 = num.parse().unwrap();
    println!("Calling: {}", val);

    for idx in 0..cards.len() {
      if cards[idx].mark(val) {
        println!("Card bingod: {}", idx);
      }
    }

    if cards.len() == 1 && cards[0].bingo {
      println!(
        "Card ID: {} Card Sum: {}, Final Value: {}, Answer: {}",
        idx,
        cards[0].sum,
        val,
        cards[0].sum * val
      );
      break;
    }

    cards.retain(|card| card.bingo == false);
  }
}
