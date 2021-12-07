fn get_fuel_cost(crabs: &Vec<isize>, pos: isize) -> isize {
  let fuel_cost = crabs.iter().fold(0, |cost, crab| cost + (crab - pos).abs());
  return fuel_cost;
}

pub fn solve() {
  let mut crabs: Vec<isize> = include_str!("input.txt")
    .trim()
    .split(',')
    .map(|crab| crab.parse().unwrap())
    .collect();
  crabs.sort_unstable();

  let max = crabs.last().unwrap();

  let mut min = 999999999999;

  for i in 0..(max + 1) {
    let cost = get_fuel_cost(&crabs, i);
    if cost < min {
      min = cost;
    }
  }
  /*
  let mut fuel_use
  [0..*max].binary_search_by(|probe| {

  });*/
  println!("Min Fuel: {}", min);
}
