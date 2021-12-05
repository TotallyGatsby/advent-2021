use crate::util::get_file_reader;
use std::io::BufRead;

mod grid;
mod lines;

pub fn solve() {
  let file = get_file_reader("./src/day05/problem2/input.txt");

  let file_lines = file.lines();

  let mut grid = grid::SeaGrid::new();

  for line in file_lines {
    let span = lines::Span::from_string(&line.unwrap());
    grid.blit(span.get_sea_cells());
  }

  // grid.print();
  let peaks = grid.get_peak_count();
  println!("Peak Count: {}", peaks);
}
