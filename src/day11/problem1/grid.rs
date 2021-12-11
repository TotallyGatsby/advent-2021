pub struct SeaGrid {
  cells: Vec<Vec<usize>>,
  pub flashes: usize,
}

impl SeaGrid {
  pub fn new(lines: Vec<&str>) -> SeaGrid {
    let mut grid = SeaGrid {
      cells: (vec![vec![]]),
      flashes: 0,
    };

    for (y, line) in lines.iter().enumerate() {
      for numeral in line.chars() {
        grid.cells[y].push(numeral.to_digit(10).unwrap() as usize);
      }
      grid.cells.push(vec![]);
    }
    grid.cells.pop();

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

  fn add_energy(&mut self, x: isize, y: isize) {
    if x < 0 || y < 0 || x >= 10 || y >= 10 {
      return;
    }

    let x_idx = x as usize;
    let y_idx = y as usize;
    self.cells[y_idx][x_idx] += 1;

    if self.cells[y_idx][x_idx] == 10 {
      self.flashes += 1;
      for offset_y in -1..2 {
        for offset_x in -1..2 {
          let neighbor_x = x + offset_x;
          let neighbor_y = y + offset_y;

          //println!("Checking: {},{}", neighbor_x, neighbor_y);
          if offset_x == 0 && offset_y == 0 {
            continue;
          }
          self.add_energy(neighbor_x, neighbor_y);
        }
      }
    }
  }

  pub fn octopus_step(&mut self) {
    // Add energy to every cell in the grid
    for y in 0..10 {
      for x in 0..10 {
        self.add_energy(x, y);
      }
    }

    // Reset anything that's flashed
    for y in 0..10 {
      for x in 0..10 {
        if self.cells[y][x] > 9 {
          self.cells[y][x] = 0;
        }
      }
    }
  }

  pub fn is_sync(&self) -> bool {
    for y in 0..10 {
      for x in 0..10 {
        if self.cells[y][x] > 0 {
          return false;
        }
      }
    }

    return true;
  }
}
