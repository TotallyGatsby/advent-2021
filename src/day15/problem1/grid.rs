use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PathState {
  cost: usize,
  position: (usize, usize),
}

impl Ord for PathState {
  fn cmp(&self, other: &Self) -> Ordering {
    // Notice that the we flip the ordering on costs (so the heap puts the smallest at the top)
    // In case of a tie we compare positions - this step is necessary
    // to make implementations of `PartialEq` and `Ord` consistent.
    other
      .cost
      .cmp(&self.cost)
      .then_with(|| self.position.cmp(&other.position))
  }
}

impl PartialOrd for PathState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

pub struct SeaGrid {
  cells: Vec<Vec<usize>>,
  costs: Vec<Vec<usize>>,
}

impl SeaGrid {
  pub fn new(lines: Vec<&str>, repeat_x: usize, repeat_y: usize) -> SeaGrid {
    let mut grid = SeaGrid {
      cells: (vec![vec![0; lines[0].len() * repeat_x]; lines.len() * repeat_y]),
      costs: (vec![vec![0; lines[0].len() * repeat_x]; lines.len() * repeat_y]),
    };

    let max_y = lines.len();
    let max_x = lines[0].trim().len();
    for y_repeat_index in 0..repeat_y {
      for x_repeat_index in 0..repeat_x {
        for (y, line) in lines.iter().enumerate() {
          for (x, numeral) in line.chars().enumerate() {
            let mut cell_val =
              numeral.to_digit(10).unwrap() as usize + y_repeat_index + x_repeat_index;
            while cell_val > 9 {
              cell_val -= 9;
            }
            grid.cells[y + max_y * y_repeat_index][x + max_x * x_repeat_index] = cell_val;
          }
        }
      }
    }

    grid.cells.truncate(max_y * repeat_y + 1);
    grid.costs.truncate(max_y * repeat_y + 1);
    grid
      .cells
      .iter_mut()
      .for_each(|row| row.truncate(max_x * repeat_x));
    grid
      .costs
      .iter_mut()
      .for_each(|row| row.truncate(max_x * repeat_x));

    grid.print();
    return grid;
  }

  pub fn get_route_cost(&mut self) -> usize {
    let mut distances = HashMap::new();

    let mut frontier = BinaryHeap::new();
    let goal = (self.cells.len() - 1, self.cells[0].len() - 1);

    frontier.push(PathState {
      cost: 0,
      position: (0, 0),
    });

    distances.insert((0, 0), 0);

    println!("Goal {:?}", goal);
    while let Some(PathState { cost, position }) = frontier.pop() {
      //println!("Searching: {:?}", position);
      if goal == position {
        return cost;
      }

      if cost > distances[&position] {
        continue;
      }

      for (x, y) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        if position.1 < 1 && y == -1 {
          continue;
        }

        if position.0 < 1 && x == -1 {
          continue;
        }

        let new_x = (position.0 as isize + x) as usize;
        let new_y = (position.1 as isize + y) as usize;

        if new_y >= self.cells.len() || new_x >= self.cells[0].len() {
          continue;
        }

        //println!("Considering: {}, {}", new_x, new_y);
        let next = PathState {
          cost: cost + self.cells[new_y][new_x],
          position: (new_x, new_y),
        };

        if !distances.contains_key(&next.position) || next.cost < distances[&next.position] {
          frontier.push(next);
          distances.insert(next.position, next.cost);
          self.costs[new_y][new_x] = next.cost;
        }
      }
    }

    0
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

  #[allow(dead_code)]
  pub fn print_costs(&self) {
    for row in self.costs.iter() {
      for col in row {
        print!("{:02} ", col);
      }
      print!("\n");
    }
  }
}
