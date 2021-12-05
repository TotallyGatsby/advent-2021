#[derive(PartialEq, Debug)]
pub enum Direction {
  Horizontal,
  Vertical,
  Diagonal,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Point {
  x: i32,
  y: i32,
}

impl Point {
  pub fn from_string(input: &str) -> Point {
    let vals = input.split(',').collect::<Vec<_>>();
    return Point {
      x: vals[0].parse().unwrap(),
      y: vals[1].parse().unwrap(),
    };
  }
}

#[derive(Debug)]
pub struct Span {
  pub dir: Direction,
  pub start: Point,
  pub end: Point,
}

// A start and end point for a line segment on the sea floor

impl Span {
  pub fn from_string(input: &String) -> Span {
    let points = input.split(" -> ").collect::<Vec<_>>();

    let point1 = Point::from_string(points[0]);
    let point2 = Point::from_string(points[1]);

    let dir;

    if point1.x == point2.x {
      dir = Direction::Vertical;
    } else if point1.y == point2.y {
      dir = Direction::Horizontal;
    } else {
      dir = Direction::Diagonal;
    }

    return Span {
      dir: dir,
      start: point1,
      end: point2,
    };
  }

  pub fn get_sea_cells(&self) -> Vec<[usize; 2]> {
    let mut sea_cells = Vec::new();

    if self.dir == Direction::Diagonal {
      return sea_cells;
    }

    let mut cur_point = self.start.clone();
    let incr;

    if (self.dir == Direction::Horizontal && self.start.x < self.end.x)
      || (self.dir == Direction::Vertical && self.start.y < self.end.y)
    {
      incr = 1;
    } else {
      incr = -1;
    }

    while cur_point != self.end {
      sea_cells.push([cur_point.x as usize, cur_point.y as usize]);

      if self.dir == Direction::Horizontal {
        cur_point.x += incr;
      } else {
        cur_point.y += incr;
      }
    }

    sea_cells.push([cur_point.x as usize, cur_point.y as usize]);

    return sea_cells;
  }
}
