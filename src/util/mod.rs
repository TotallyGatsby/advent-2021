use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn get_file_reader(input_path: &str) -> std::io::BufReader<std::fs::File> {
  let path = Path::new(input_path);

  let display = path.display();

  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
  };

  let file_reader = BufReader::new(file);

  return file_reader;
}
