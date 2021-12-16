pub mod grid;

#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("sample.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let mut sea_grid = grid::SeaGrid::new(lines, 1, 1);

  println!("Path Cost: {}", sea_grid.get_route_cost());
}
