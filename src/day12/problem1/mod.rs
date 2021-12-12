pub mod graph;

#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let caves = graph::Graph::new(lines);

  println!("Caves: {:?}", caves);
  println!("Paths: {}", caves.paths);
}
