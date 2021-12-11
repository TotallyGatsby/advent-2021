pub mod grid;

#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let mut grid = grid::SeaGrid::new(lines);

  for i in 0..500 {
    //println!("Step {}:", i + 1);
    grid.octopus_step();
    //grid.print();
    //println!("Flashes so far: {}", grid.flashes);
    if grid.is_sync() {
      println!("Synchronized on step {}!", i + 1);
    }
  }
}
