use std::collections::{HashSet, VecDeque};

pub struct SeaGrid {
  cells: Vec<Vec<usize>>,
  low_points: Vec<(usize, usize)>,
}

impl SeaGrid {
  pub fn new(lines: Vec<&str>) -> SeaGrid {
    let mut grid = SeaGrid {
      cells: (vec![vec![]]),
      low_points: vec![],
    };

    for (y, line) in lines.iter().enumerate() {
      for numeral in line.chars() {
        grid.cells[y].push(numeral.to_digit(10).unwrap() as usize);
      }
      grid.cells.push(vec![]);
    }
    grid.cells.pop();

    grid.get_low_points();

    return grid;
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    for row in self.cells.iter() {
      for col in row {
        print!("{}", col);
      }
      print!("\n");
    }
  }

  pub fn get_cell_height(&self, x: usize, y: usize) -> usize {
    if x < self.cells[0].len() && y < self.cells.len() {
      return self.cells[y][x];
    }

    return 99;
  }

  fn get_low_points(&mut self) {
    for (y, cells) in self.cells.iter().enumerate() {
      for (x, height) in cells.iter().enumerate() {
        let mut is_low_point = true;

        if x > 0 {
          is_low_point &= self.get_cell_height(x - 1, y) > *height;
        }

        if y > 0 {
          is_low_point &= self.get_cell_height(x, y - 1) > *height;
        }

        is_low_point &= self.get_cell_height(x + 1, y) > *height;
        is_low_point &= self.get_cell_height(x, y + 1) > *height;

        if is_low_point {
          self.low_points.push((x, y));
        }
      }
    }
  }

  pub fn get_risk(&self) -> usize {
    let mut risk = 0;

    for point in self.low_points.iter() {
      risk += self.cells[point.1][point.0] + 1;
    }

    return risk;
  }

  fn get_basin_size(&self, start_x: usize, start_y: usize) -> usize {
    let mut total_size = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut visit_queue = VecDeque::new();

    visited.insert((start_x, start_y));
    visit_queue.push_back((start_x, start_y));

    while visit_queue.len() > 0 {
      let (x, y) = visit_queue.pop_front().unwrap();
      if y >= self.cells.len() || x >= self.cells[0].len() || self.cells[y][x] == 9 {
        continue; // Ignore peaks and oob requests
      }

      // Count the cell
      total_size += 1;

      // Enqueue neighbors for counting
      if x > 0 && !visited.contains(&(x - 1, y)) {
        visited.insert((x - 1, y));
        visit_queue.push_back((x - 1, y))
      }
      if y > 0 && !visited.contains(&(x, y - 1)) {
        visited.insert((x, y - 1));
        visit_queue.push_back((x, y - 1))
      }
      if !visited.contains(&(x + 1, y)) {
        visited.insert((x + 1, y));
        visit_queue.push_back((x + 1, y))
      }
      if !visited.contains(&(x, y + 1)) {
        visited.insert((x, y + 1));
        visit_queue.push_back((x, y + 1))
      }
    }
    return total_size;
  }

  pub fn get_basins(&self) -> Vec<usize> {
    let mut basins = vec![];

    for point in self.low_points.iter() {
      basins.push(self.get_basin_size(point.0, point.1));
    }
    basins.sort_unstable();
    basins.reverse();
    return basins;
  }
}
