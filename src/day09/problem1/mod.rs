pub mod grid;

#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let grid = grid::SeaGrid::new(lines);
  let basins = grid.get_basins();
  println!("Total Risk: {}", grid.get_risk());
  println!("Basin Sizes: {:?}", basins);
  println!("Basin Calculation: {}", basins[0] * basins[1] * basins[2]);
}
