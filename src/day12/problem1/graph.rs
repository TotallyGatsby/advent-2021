use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Graph {
  pub paths: usize,
  pub nodes: HashMap<String, Vec<String>>,
}

impl Graph {
  pub fn new(lines: Vec<&str>) -> Graph {
    let mut tmp = Graph {
      paths: 0,
      nodes: HashMap::new(),
    };

    for line in lines {
      let splits = line.split('-').collect::<Vec<_>>();
      let left = splits[0].to_string();
      let right = splits[1].to_string();

      // Create bidirectional connections
      tmp
        .nodes
        .entry(left.clone())
        .or_insert(Vec::new())
        .push(right.clone());
      tmp.nodes.entry(right).or_insert(Vec::new()).push(left);
    }

    tmp.find_paths();

    return tmp;
  }

  fn walk_caves(&mut self, cell: String, path: &mut VecDeque<String>, revisited: &mut bool) {
    if cell == "end" {
      self.paths += 1;
      return;
    }

    let is_small_cell = cell.chars().any(|x| x.is_ascii_lowercase());
    path.push_back(cell.clone());
    let mut revisits = 0;
    if is_small_cell {
      revisits = path
        .iter()
        .fold(0, |acc, item| if item == &cell { acc + 1 } else { acc });
    }

    if revisits >= 2 {
      *revisited = true;
    }

    let nodes = self.nodes[&cell].clone();

    for neighbor in nodes {
      if neighbor.chars().any(|x| x.is_ascii_lowercase()) && path.contains(&neighbor) && *revisited
      {
        continue;
      }

      if neighbor == "start" {
        continue;
      }

      self.walk_caves(neighbor.clone(), path, revisited);
    }

    if revisits >= 2 {
      *revisited = false;
    }
    path.pop_back();
  }

  pub fn find_paths(&mut self) {
    self.walk_caves("start".to_string(), &mut VecDeque::new(), &mut false);
    // If the current node is end, add one to paths and complete
    // for each neighbor node
    // If it's a small cave and visited, continue
    // Recurse
  }
}
