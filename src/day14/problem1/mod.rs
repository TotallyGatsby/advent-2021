use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let mut polymer = lines[0].to_string();

  let mut rules: HashMap<String, String> = HashMap::new();

  for rule in lines.iter().skip(2) {
    let rule_pieces = rule.split("->").collect::<Vec<_>>();
    rules.insert(
      rule_pieces[0].trim().to_string(),
      rule_pieces[1].trim().to_string(),
    );
  }

  println!("Polymer: {}", polymer);
  for i in 0..10 {
    println!("Inserting, Step {}", i + 1);
    polymer = polymer.chars().collect::<Vec<_>>().windows(2).fold(
      polymer.chars().nth(0).unwrap().to_string(),
      |result, pair| {
        let lookup: String = pair.iter().collect();
        let insert_char = &rules[&lookup];

        result + &insert_char + &pair[1].to_string()
      },
    );
  }

  let mut counts = HashMap::new();
  for cha in polymer.chars() {
    *counts.entry(cha).or_insert(0usize) += 1;
  }

  let mut values = counts.values().collect::<Vec<_>>();
  values.sort_unstable();

  println!("Values: {:?}", values);
  println!("Answer: {}", *values.last().unwrap() - values[0]);
}
