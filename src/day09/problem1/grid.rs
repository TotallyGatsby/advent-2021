pub struct SeaGrid {
  cells: Vec<Vec<usize>>,
}

impl SeaGrid {
  pub fn new(lines: Vec<&str>) -> SeaGrid {
    let mut grid = SeaGrid {
      cells: (vec![vec![]]),
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

  pub fn get_cell_height(&self, x: usize, y: usize) -> usize {
    if x < self.cells[0].len() && y < self.cells.len() {
      return self.cells[y][x];
    }

    return 99;
  }

  pub fn get_risk(&self) -> usize {
    let mut risk = 0;

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
          risk += height + 1;
        }
      }
    }
    return risk;
  }

  #[allow(dead_code)]
  pub fn get_peak_count(&self) -> usize {
    let mut peak_count = 0;

    for row in self.cells.iter() {
      for col in row {
        if *col > 1 {
          peak_count += 1;
        }
      }
    }

    return peak_count;
  }
}
