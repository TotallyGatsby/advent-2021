pub mod grid;

#[allow(dead_code)]
pub fn solve() {
  let lines = include_str!("input.txt")
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

  let coords = lines
    .iter()
    .filter(|line| line.contains(','))
    .collect::<Vec<_>>();

  let commands = lines
    .iter()
    .filter(|line| line.contains("fold"))
    .map(|command| {
      let tmp = command.split('=').collect::<Vec<_>>();
      let split_coord: usize = tmp[1].parse().unwrap();
      let split_angle = tmp[0].chars().last().unwrap();
      (split_angle, split_coord)
    })
    .collect::<Vec<_>>();

  println!("{:?}", commands);
  let mut fold_grid = grid::SeaGrid::new(coords, 2000);

  for command in commands {
    fold_grid.fold(command.0, command.1);
    println!(
      "Folded: {}, {}\tDots: {}",
      command.0,
      command.1,
      fold_grid.count_dots()
    );
  }

  fold_grid.print();
  /*
  fold_grid.print();

  fold_grid.fold('y', 7);

  println!("Folded:");
  fold_grid.print();
  fold_grid.fold('x', 5);

  println!("Folded:");
  fold_grid.print();
  */
}
