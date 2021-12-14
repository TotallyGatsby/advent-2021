// use std::collections::{HashSet, VecDeque};

pub struct SeaGrid {
  cells: Vec<Vec<bool>>,
}

impl SeaGrid {
  pub fn new(lines: Vec<&&str>, size: usize) -> SeaGrid {
    let mut grid = SeaGrid {
      cells: (vec![vec![false; size]; lines.len()]),
    };

    let mut max_y = 0;
    let mut max_x = 0;
    for line in lines {
      let coords = line.split(',').collect::<Vec<_>>();

      let x: usize = coords[0].trim().parse().unwrap();
      let y: usize = coords[1].trim().parse().unwrap();

      if y > max_y {
        max_y = y;
      }
      if x > max_x {
        max_x = x;
      }

      grid.cells[y][x] = true;
    }

    grid.cells.truncate(max_y + 1);
    grid
      .cells
      .iter_mut()
      .for_each(|row| row.truncate(max_x + 1));
    return grid;
  }

  fn fold_y(&mut self, plane: usize) {
    for i in plane..self.cells.len() {
      if i > plane * 2 {
        break;
      }

      let offset = i - plane;
      for x in 0..self.cells[i].len() {
        self.cells[plane - offset][x] |= self.cells[i][x];
      }
    }

    self.cells.truncate(plane);
  }

  fn fold_x(&mut self, plane: usize) {
    for i in 0..self.cells.len() {
      for x in plane..self.cells[i].len() {
        if x > plane * 2 {
          break;
        }
        let offset = x - plane;
        self.cells[i][plane - offset] |= self.cells[i][x];
      }
    }

    self.cells.iter_mut().for_each(|row| row.truncate(plane));
  }

  pub fn fold(&mut self, dir: char, plane: usize) {
    if dir == 'y' {
      self.fold_y(plane);
    } else if dir == 'x' {
      self.fold_x(plane);
    }
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    for row in self.cells.iter() {
      for col in row {
        if *col {
          print!("#");
        } else {
          print!(".");
        }
      }
      print!("\n");
    }
  }

  pub fn count_dots(&self) -> usize {
    let mut dots = 0;
    for row in self.cells.iter() {
      for col in row {
        if *col {
          dots += 1;
        }
      }
    }
    return dots;
  }
}
