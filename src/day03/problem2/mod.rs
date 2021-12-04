use crate::util::get_file_reader;
use std::io::BufRead;

// Gets the halfway point, using the integer math and rules of the problem
fn get_midpoint(min: usize, max: usize, round_up: bool) -> usize {
  let mut span = max - min;
  if round_up {
    span += 1;
  }
  return (min + (span / 2)) as usize;
}

fn get_partition_index(char_index: usize, min: usize, max: usize, lines: &Vec<String>) -> usize {
  let slice = &lines[min..max];
  /*
  println!(
    "Evaluating slice between {} and {} (char index: {}): {:?}",
    min, max, char_index, slice
  );
  */

  // Use partition to find the first element with a '1' at character digit 'char_index'
  let i =
    slice.partition_point(|binary_string| binary_string.chars().nth(char_index).unwrap() == '0');

  return i + min;
}

// Converts a vector of 1s and 0s into a single decimal number
fn binstr_2_dec(binstr: &String) -> i32 {
  return i32::from_str_radix(binstr, 2).unwrap();
}

#[allow(dead_code)]
pub fn solve() {
  let file_reader = get_file_reader("./src/day03/problem2/input.txt");

  // Get our input and sort it, so we can use two pointers to 'zero in' on our target
  let mut lines: Vec<String> = file_reader.lines().collect::<Result<_, _>>().unwrap();
  lines.sort_unstable(); // unstable sort is ok, equivalent values can be in any order

  // Loop variables
  let string_len = lines[0].len();
  let mut char_index = 0;
  let mut index; // get_partition_index(0, oxy_min, oxy_max, &lines);
  let mut midpoint; //  get_midpoint(oxy_min, oxy_max);

  // Hold two indices to the min and max value that oxygen could be
  let mut oxy_min = 0;
  let mut oxy_max = lines.len() - 1;

  // Loop through each column of the data, determine if there are more ones or zeros
  // Because the array is sorted, we can find the index of the first element that's non-zero
  // and compare that to the 'halfway' point between the current known min and max
  while (oxy_min + 1) != oxy_max && char_index < string_len {
    // Find the index of the first entry with a '1' at position 'char_index'
    index = get_partition_index(char_index, oxy_min, oxy_max, &lines);
    // Find the midpoint of the current min<=>max window
    midpoint = get_midpoint(oxy_min, oxy_max, true);

    /*
    println!(
      "Found partition index: {} ({}) and oxy min/max/midpoint: {}/{}/{} ({})",
      index, lines[index], oxy_min, oxy_max, midpoint, lines[midpoint]
    );
    */

    // Adjust the min or max, depending on which shows up more
    if midpoint < index {
      oxy_max = index;
    } else if midpoint == index && (oxy_max - oxy_min) % 2 != 0 {
      oxy_max = index;
    } else {
      oxy_min = index;
    }

    char_index += 1;
  }

  // Repeat a similar process for CO2
  let mut co2_min = 0;
  let mut co2_max = lines.len() - 1;
  char_index = 0;
  while co2_min != co2_max && char_index < string_len {
    index = get_partition_index(char_index, co2_min, co2_max, &lines);
    midpoint = get_midpoint(co2_min, co2_max, false);

    /*
    println!(
      "Found partition index: {} and CO2 midpoint: {}",
      index, midpoint
    );
    */
    if index <= midpoint {
      co2_max = index;
    } else {
      co2_min = index;
    }

    char_index += 1;
  }

  println!(
    "Final Oxy: {}, Dec Value: {}",
    lines[oxy_min],
    binstr_2_dec(&lines[oxy_min])
  );
  println!(
    "Final CO2: {}, Dec Value: {}",
    lines[co2_min],
    binstr_2_dec(&lines[co2_min])
  );

  println!(
    "Answer: {}",
    binstr_2_dec(&lines[co2_min]) * binstr_2_dec(&lines[oxy_min])
  );
}
