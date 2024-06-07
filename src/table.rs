use std::io;
use std::fs;

/** Holds data from a `csv` in a table. Data is oriented in a column-major fashion. Doesn't hold the atomic number, because that is already encoding by the index (atomic number = 1 + index). */
#[derive(Debug)]
pub struct ElementTable {
  length: usize,
  data: (Vec<String>, Vec<String>, Vec<f64>)
}

impl ElementTable {
  pub fn from_csv(file_name: &str) -> io::Result<Self> {
    let mut length = 0;
    let mut data = (vec![], vec![], vec![]);
    
    for line in fs::read_to_string(file_name)
      .expect("Failed to read file")
      .to_ascii_lowercase()
      .lines()
      .skip(1)
    {
      let entry = line.split(",").collect::<Vec<&str>>();
      data.0.push(entry[1].to_owned());
      data.1.push(entry[2].to_owned());
      data.2.push(entry[3].parse::<f64>().expect("Found error when reading csv"));
      length += 1;
    }

    Ok(Self { length, data })
  }

  /** Gets index in the given column vector or returns `None`.*/
  pub fn column_get_idx<T: std::cmp::Eq>(column: &Vec<T>, val: T) -> Option<usize> {
    column.iter().position(|v| *v == val)
  }

  pub fn length(&self) -> usize {
    self.length
  }

  pub fn data(&self) -> &(Vec<String>, Vec<String>, Vec<f64>) {
    &self.data
  }

  pub fn categories(&self) -> [&'static str; 4] {
    ["Atomic Number", "Name", "Symbol", "Atomic Weight"]
  }
}