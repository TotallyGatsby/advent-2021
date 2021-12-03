use crate::util::get_lines;

fn initialize_vector(sums: &mut Vec<i16>, count: usize) {
  for _ in 0..count {
    sums.push(0);
  }
}

// Converts a vector of 1s and 0s into a single decimal number
fn vec_2_dec(vals: &Vec<i8>) -> i32 {
  let mut dec: i32 = 0;

  for val in vals.iter() {
    dec = dec * 2;
    dec += *val as i32;
  }

  return dec;
}

#[allow(dead_code)]
pub fn solve() {
  let lines = get_lines("./src/day03/problem1/input.txt");

  let mut sums: Vec<i16> = Vec::new();

  let mut processed_count = 0;
  for line in lines {
    if let Ok(binary_string) = line {
      // Initialize our vector with 0s
      if sums.len() == 0 {
        initialize_vector(&mut sums, binary_string.chars().count());
      }

      let digits = binary_string.char_indices();
      for digit in digits {
        // If we see a one, increase the counter for that digit in our gamma rate
        if digit.1 == '1' {
          sums[digit.0] += 1;
        }
      }
      processed_count += 1;
    }
  }
  // If the number of 1s we saw is greater than half the number of inputs, then 1s are more common
  processed_count = processed_count / 2;
  let gamma: Vec<i8> = sums
    .iter()
    .map(|sum| if sum > &processed_count { 1 } else { 0 })
    .collect();

  let epsilon: Vec<i8> = sums
    .iter()
    .map(|sum| if sum < &processed_count { 1 } else { 0 })
    .collect();

  let gamma_val = vec_2_dec(&gamma);
  let epsilon_val = vec_2_dec(&epsilon);
  println!("Resulting sums: {:?}", sums);
  println!("Gamma: {:?} (dec: {})", gamma, gamma_val);
  println!("Epsilon: {:?} (dec: {})", epsilon, epsilon_val);
  println!("Power Consumption: {}", gamma_val * epsilon_val);
}
