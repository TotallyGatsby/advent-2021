use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let polymer = lines[0].to_string();

  let mut rules = HashMap::new();

  for rule in lines.iter().skip(2) {
    let rule_pieces = rule.split("->").collect::<Vec<_>>();
    rules.insert(
      rule_pieces[0].trim().to_string(),
      rule_pieces[1].trim().to_string(),
    );
  }

  let mut bigrams: HashMap<String, usize> = HashMap::new();

  for rule in rules.keys() {
    bigrams.insert(rule.to_string(), 0);
  }

  for gram in polymer.chars().collect::<Vec<_>>().windows(2) {
    *bigrams
      .entry(gram.iter().collect::<String>())
      .or_insert(0usize) += 1;
  }

  println!("Bigrams: {:?}", bigrams);
  for i in 0..40 {
    println!("Iterating Step {}", i + 1);

    let mut new_values = HashMap::new();
    for rule in &rules {
      *new_values
        .entry(rule.0.chars().nth(0).unwrap().to_string() + &rule.1.to_string())
        .or_insert(0usize) += *bigrams.entry(rule.0.to_string()).or_insert(0usize);
      *new_values
        .entry(rule.1.to_string() + &rule.0.chars().nth(1).unwrap().to_string())
        .or_insert(0usize) += *bigrams.entry(rule.0.to_string()).or_insert(0usize);
    }
    bigrams = new_values;
  }

  let mut counts = HashMap::new();
  for bigram in bigrams {
    *counts
      .entry(bigram.0.chars().nth(0).unwrap())
      .or_insert(0usize) += bigram.1;
  }

  *counts
    .entry(polymer.chars().last().unwrap())
    .or_insert(0usize) += 1;

  let mut values = counts.values().collect::<Vec<_>>();
  values.sort_unstable();

  println!("Answer: {}", *values.last().unwrap() - values[0]);
}
