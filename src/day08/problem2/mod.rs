use std::collections::HashMap;

fn bitform_word(word: &str) -> u8 {
  let mut result = 0;

  for char in word.chars() {
    if char == 'a' {
      result |= 1;
    } else if char == 'b' {
      result |= 2;
    } else if char == 'c' {
      result |= 4;
    } else if char == 'd' {
      result |= 8;
    } else if char == 'e' {
      result |= 16;
    } else if char == 'f' {
      result |= 32;
    } else if char == 'g' {
      result |= 64;
    } else {
      println!("UNKNOWN CHARACTER: {}", char);
    }
  }
  return result;
}

fn get_sorted_word(word: &str) -> String {
  let mut chars = word.chars().collect::<Vec<_>>();

  chars.sort_unstable_by(|a, b| a.cmp(b));
  let result: String = chars.iter().collect::<String>();
  return result;
}

fn digitize_left_side(lhs: &str) -> HashMap<String, u8> {
  let mut words = lhs.split(' ').collect::<Vec<_>>();

  words.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

  let mut encoded_digits: HashMap<String, u8> = HashMap::new();
  let mut bitforms: [u8; 10] = [0; 10];

  // Handle digit 1
  encoded_digits.insert(get_sorted_word(words[0]), 1);
  bitforms[1] = bitform_word(words[0]);

  // Handle digit 7
  encoded_digits.insert(get_sorted_word(words[1]), 7);
  bitforms[7] = bitform_word(words[1]);

  // Handle digit 4
  encoded_digits.insert(get_sorted_word(words[2]), 4);
  bitforms[4] = bitform_word(words[2]);

  // Handle digit 8
  encoded_digits.insert(get_sorted_word(words[9]), 8);
  bitforms[8] = bitform_word(words[9]);

  let mut six_digit_words = words[6..9].iter().cycle();

  // Find digits 0, 6, and 9
  while bitforms[0] == 0 || bitforms[6] == 0 || bitforms[9] == 0 {
    let cur_word = six_digit_words.next().unwrap();

    if encoded_digits.contains_key(&get_sorted_word(cur_word)) {
      continue;
    }

    let cur_bitform = bitform_word(cur_word);

    // Check for digit six if we don't have it already
    if bitforms[6] == 0 {
      if cur_bitform & bitforms[1] != bitforms[1] {
        encoded_digits.insert(get_sorted_word(cur_word), 6);
        bitforms[6] = cur_bitform;
        continue;
      }
    }

    // Check for digit 9
    if bitforms[9] == 0 {
      if cur_bitform & bitforms[4] == bitforms[4] {
        encoded_digits.insert(get_sorted_word(cur_word), 9);
        bitforms[9] = cur_bitform;
      }
      continue;
    }

    // The last digit must be 0
    if bitforms[6] != 0 && bitforms[9] != 0 {
      encoded_digits.insert(get_sorted_word(cur_word), 0);
      bitforms[0] = cur_bitform;
    }
  }

  let mut five_digit_words = words[3..6].iter().cycle();

  let sixty_niner = bitforms[6] ^ bitforms[9];

  // Find digits 0, 6, and 9
  while bitforms[2] == 0 || bitforms[3] == 0 || bitforms[5] == 0 {
    let cur_word = five_digit_words.next().unwrap();

    if encoded_digits.contains_key(&get_sorted_word(cur_word)) {
      continue;
    }

    let cur_bitform = bitform_word(cur_word);

    // Check for digit three if we don't have it already
    if bitforms[3] == 0 {
      if cur_bitform & bitforms[1] == bitforms[1] {
        encoded_digits.insert(get_sorted_word(cur_word), 3);
        bitforms[3] = cur_bitform;
        continue;
      }
    }

    // Check for digit 2
    if bitforms[2] == 0 {
      if cur_bitform & sixty_niner == sixty_niner {
        encoded_digits.insert(get_sorted_word(cur_word), 2);
        bitforms[2] = cur_bitform;
      }
      continue;
    }

    // The last digit must be 0
    if bitforms[2] != 0 && bitforms[3] != 0 {
      encoded_digits.insert(get_sorted_word(cur_word), 5);
      bitforms[5] = cur_bitform;
    }
  }

  /*
  println!("Sorted words: {:?}", words);
  println!("Bitformed words: {:?}", bitforms);
  println!("Word digits: {:?}", encoded_digits);
  */

  return encoded_digits;
}

pub fn solve() {
  let messages = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let mut total_sum = 0;
  for message in messages {
    let sides = message.split('|').collect::<Vec<_>>();
    let left_side = sides[0].trim();
    let right_side_words = sides[1].trim().split(' ').collect::<Vec<_>>();

    let lookups = digitize_left_side(left_side);
    let mut sum: usize = 0;

    for word in right_side_words {
      let sorted_word = get_sorted_word(word);
      sum *= 10;
      sum += lookups[&sorted_word] as usize;
      //print!("{} ({}) ", word, lookups[&sorted_word]);
    }

    //print!(" Sum: {} \n", sum);

    total_sum += sum;
  }

  println!("Total Sum: {}", total_sum);
}
