#[allow(dead_code)]
pub fn solve() {
  let messages = include_str!("input.txt").split('\n').collect::<Vec<_>>();

  let mut count = 0;
  for message in messages {
    if message.len() == 0 {
      continue;
    }
    let right_side = message.split('|').collect::<Vec<_>>()[1].trim();
    let words = right_side.split(' ').collect::<Vec<_>>();

    count += words.iter().fold(0, |acc, word| {
      let wordlen = word.len();

      if wordlen == 2 || wordlen == 4 || wordlen == 3 || wordlen == 7 {
        return acc + 1;
      } else {
        return acc;
      }
    });
  }

  println!("Count of known digits: {}", count);
}
