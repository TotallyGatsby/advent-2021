#[allow(dead_code)]
pub fn solve() {
  let fish_ages = include_str!("input.txt").trim().split(',');
  let mut pops: [u64; 9] = [0; 9];

  for age in fish_ages {
    let age_num: usize = age.parse().unwrap();
    pops[age_num] += 1;
  }

  for _ in 0..256 {
    pops.rotate_left(1);
    pops[6] += pops[8];
  }

  println!("Total Pop: {}", pops.iter().sum::<u64>());
}
