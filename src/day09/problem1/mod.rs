pub mod grid;

#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let grid = grid::SeaGrid::new(lines);

  println!("Total Risk: {}", grid.get_risk());
}
