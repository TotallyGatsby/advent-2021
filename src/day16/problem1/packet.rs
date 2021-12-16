type BitSlice<'a> = &'a bitvec::slice::BitSlice<bitvec::order::Msb0, u8>;

#[derive(Clone, Debug, PartialEq)]
enum PacketType {
  Literal,
  Operator,
}

#[derive(Debug)]
pub struct Packet {
  version: u8,
  msg_type: PacketType,
  children: Vec<Packet>,
}

impl Packet {
  pub fn new(line: &str) -> Self {
    let bin_line = hex::decode(line).unwrap();
    let mut input = bitvec::view::BitView::view_bits(bin_line.as_slice());

    return Packet::decode(&mut input);
  }

  fn decode(input: &mut BitSlice) -> Self {
    let version = get_bits_u8(input, 3);
    println!("Version: {}", version);

    let msg_type_val = get_bits_u8(input, 3);
    println!("Msg Type ID: {}", msg_type_val);

    let msg_type;
    if msg_type_val == 4 {
      msg_type = PacketType::Literal;
    } else {
      msg_type = PacketType::Operator;
    }
    println!("Msg Type: {:?}", msg_type);

    let mut children = Vec::new();

    if msg_type == PacketType::Literal {
      let literal = decode_literal(input);
      println!("Literal: {}", literal);
    } else if msg_type == PacketType::Operator {
      children = Self::parse_subpackets(input);
    }

    Self {
      version,
      msg_type,
      children,
    }
  }

  fn parse_subpackets(mut input: &mut BitSlice) -> Vec<Packet> {
    let (length_type, rem) = input.split_at(1);
    *input = rem;

    let mut sub_packets = Vec::new();
    if length_type[0] {
      // Length Type ID 1
      let len = get_bits_usize(input, 11);
      println!("Length Type ID 1, Packets to read: {}", len);
      for _ in 0..len {
        sub_packets.push(Packet::decode(&mut input));
      }
    } else {
      // Length Type ID 0
      // Get 15 bits out
      let len = get_bits_usize(input, 15);

      println!("Length Type ID 0, Bits to read: {} of {}", len, input.len());
      let (mut sub_packet_input, remainder) = input.split_at(len);

      *input = remainder;

      while !sub_packet_input.is_empty() {
        println!("Processing subpacket: {}", sub_packet_input);
        sub_packets.push(Packet::decode(&mut sub_packet_input));
      }
    }

    println!("Found sub packets: {:?}", sub_packets);
    sub_packets
  }

  pub fn get_version_sum(&self) -> usize {
    let mut sum = self.version as usize;

    for child in &self.children {
      sum += child.get_version_sum();
    }
    sum
  }
}

fn get_bits_usize(bit_slice: &mut BitSlice, count: usize) -> usize {
  let (first, last) = bit_slice.split_at(count);

  *bit_slice = last;

  slice_to_usize(first)
}

fn slice_to_usize(bit_slice: BitSlice) -> usize {
  let mut result = 0;

  for bit in bit_slice {
    result = result << 1 | if *bit { 1 } else { 0 };
  }

  result
}

fn get_bits_u8(bit_slice: &mut BitSlice, count: usize) -> u8 {
  let (first, last) = bit_slice.split_at(count);

  *bit_slice = last;

  slice_to_usize(first) as u8
}

fn decode_literal(input: &mut BitSlice) -> usize {
  let (mut nibble, mut last) = input.split_at(5);

  let mut result = 0;

  while nibble[0] {
    let (_, bytes) = nibble.split_at(1);
    result = result << 4 | slice_to_usize(bytes);

    *input = last;
    let next_split = input.split_at(5);
    nibble = next_split.0;
    last = next_split.1;
  }

  let (_, bytes) = nibble.split_at(1);
  result = result << 4 | slice_to_usize(bytes);
  *input = last;

  result
}
