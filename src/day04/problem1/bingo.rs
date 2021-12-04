pub struct Card {
  pub sum: u16,
  rows: [Row; 10],
}

/* Bing card looks like this:
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
 */
impl Card {
  pub fn from_lines(lines: &[String]) -> Card {
    let mut new_card = Card {
      sum: 0,
      rows: [Row::default(); 10],
    };

    if lines.len() != 5 {
      panic!(
        "Passed incorrect number of lines for bingo card! {:?}",
        lines
      );
    }

    // Create our horizontal rows
    let mut idx = 0;
    for line in lines {
      // println!("Parsing: {}", line);
      let number1: u16 = line[0..2].trim().parse().unwrap();
      let number2: u16 = line[3..5].trim().parse().unwrap();
      let number3: u16 = line[6..8].trim().parse().unwrap();
      let number4: u16 = line[9..11].trim().parse().unwrap();
      let number5: u16 = line[12..14].trim().parse().unwrap();

      // println!("Found: {:?}", [number1, number2, number3, number4, number5]);
      new_card.rows[idx] = Row::from_array([number1, number2, number3, number4, number5]);
      new_card.sum += number1 + number2 + number3 + number4 + number5;
      idx += 1;
    }
    // Create our vertical rows
    for col in 0..5 {
      new_card.rows[idx] = Row::from_array([
        new_card.rows[0].values[col],
        new_card.rows[1].values[col],
        new_card.rows[2].values[col],
        new_card.rows[3].values[col],
        new_card.rows[4].values[col],
      ]);
      idx += 1;
    }

    /*
        println!(
          "Built a card with a sum of {}\n{:?}",
          new_card.sum, new_card.rows
        );
    */
    return new_card;
  }

  // Marks the board with number, scratching it off the board and returns true if a Bingo is found
  pub fn mark(&mut self, number: u16) -> bool {
    let mut marked = false;
    let mut bingo_found = false;

    let mut prin = false;
    for row in &mut self.rows {
      marked |= (*row).mark(number);
      bingo_found |= row.found_count == 5;
      if bingo_found && !prin {
        println!("{:?}", row);
        prin = true;
      }
    }

    if marked {
      self.sum -= number;
    }

    if bingo_found {
      println!("BINGO!");
    }
    return bingo_found;
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Row {
  found_count: u8,
  values: [u16; 5],
}

impl Row {
  fn from_array(values: [u16; 5]) -> Row {
    return Row {
      found_count: 0,
      values,
    };
  }

  pub fn mark(&mut self, number: u16) -> bool {
    // println!("Looking for {} in {:?}", number, self.values);
    if self.values.into_iter().any(|x| x == number) {
      self.found_count = self.found_count + 1;

      return true;
    }

    // println!("Not found!");
    return false;
  }
}

impl Default for Row {
  fn default() -> Row {
    return Row {
      found_count: 0,
      values: [0; 5],
    };
  }
}
