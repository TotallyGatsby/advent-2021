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

    let mut cur_point = self.start.clone();

    let mut x_incr = 0;
    let mut y_incr = 0;

    if self.start.x < self.end.x {
      x_incr = 1;
    } else if self.start.x > self.end.x {
      x_incr = -1;
    }

    if self.start.y < self.end.y {
      y_incr = 1;
    } else if self.start.y > self.end.y {
      y_incr = -1;
    }

    while cur_point != self.end {
      sea_cells.push([cur_point.x as usize, cur_point.y as usize]);

      cur_point.x += x_incr;
      cur_point.y += y_incr;
    }

    sea_cells.push([cur_point.x as usize, cur_point.y as usize]);

    return sea_cells;
  }
}
