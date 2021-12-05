pub struct SeaGrid {
  cells: Vec<[u32; 1000]>,
}

impl SeaGrid {
  pub fn new() -> SeaGrid {
    return SeaGrid {
      cells: vec![[0; 1000]; 1000],
    };
  }

  pub fn blit(&mut self, span_cells: Vec<[usize; 2]>) {
    for cell in span_cells {
      self.cells[cell[1]][cell[0]] += 1;
    }
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    for row in self.cells.iter() {
      for col in *row {
        print!("{}", col);
      }
      print!("\n");
    }
  }

  pub fn get_peak_count(&self) -> u32 {
    let mut peak_count = 0;

    for row in self.cells.iter() {
      for col in *row {
        if col > 1 {
          peak_count += 1;
        }
      }
    }

    return peak_count;
  }
}
