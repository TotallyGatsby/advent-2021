pub mod packet;

#[allow(dead_code)]
pub fn solve() {
  let line = include_str!("input.txt").trim();

  let my_packet = packet::Packet::new(line);

  println!("Final Packet: {:?}", my_packet);
  println!("Version Sum: {}", my_packet.get_version_sum());
  println!("Parsed: {}", my_packet.parse_packet());
}
