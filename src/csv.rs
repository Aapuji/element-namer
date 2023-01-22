use std::fmt;

/**
A struct representing and storing csv data.

First item of each entry should have a unique "id" is the first element in the csv line, though most methods don't check for it.
*/
#[derive(Debug)]
pub struct CSV<'a> {
  categories: Vec<&'a str>,
  data: Vec<Vec<&'a str>>
}

impl<'a> CSV<'a> {
  /**
    Creates an empty `CSV` instance. 
  */
  pub fn new() -> Self {
    CSV {
      categories: vec![],
      data: vec![]
    }
  }
  
  /**
    Creates a `CSV` instance from a header (categories separated by commas). 
  */
  pub fn from(header: &'a str) -> Self {
    let categories: Vec<&'a str> = header.split(',').collect();
    let data: Vec<Vec<&'a str>> = vec![];
    
    CSV {
      categories,
      data
    }
  }

  /**
    Creates a `CSV` instance from a string, with the first line being the header, and following lines being data.

    Returns `Ok` with a `CSV` instance, or `Err` with an error message.
  */
  pub fn from_str(src: &'a str) -> Result<Self, &'static str> {
    let index = match src.find('\n'){
      Some(val) => val,
      None => return Err("Invalid source string, no line break character.")
    };

    let mut csv = CSV::from(&src[..index]);
    
    match csv.read_str(&src[index+1..]) {
      Ok(_) => Ok(csv),
      Err(_) => Err("Error occurred when attempting to read lines.")
    }
  }
  
  /**
    Reads in a line of values of data.

    If the number of values is not equal to the number of categories, then it returns an `Err` with a message. If it is a success, it returns an immutable reference to the corresponding vector in the `CSV` object.
  */
  pub fn read_line(&mut self, line: &'a str) -> Result<&Vec<&'a str>, &'static str> {
    let words: Vec<&'a str> = line.split(',').collect();
    
    if words.len() != self.categories.len() {
      return Err("Wrong number of items in line (doesn't match category size).");
    }
    
    self.data.push(words);
    
    Ok(&self.data[self.data.len() - 1])
  }
  
  /**
    Given the identifier (first item of the row), this will return either `Some` with a reference to the row, or `None` if a row with that identifier doesn't exist. 
  */
  pub fn get_row_from_id(&self, id: &str) -> Option<&Vec<&'a str>> {
    for i in 0..self.data.len() {
      if self.data[i][0] == id {
        return Some(&self.data[i]);
      }
    }
    
    None
  }
  
  /**
    Gets a string slice from the given row and category. `row` does not have to be in the `CSV` instance, but it should be correctly formatted.

    If `category` is not valid, then it will return `Err` with a message. If the length of `row` is less than the number of categories in the `CSV` instance, then it will also return an `Err`. Otherwise, it will return `Ok` with the string slice.

    Note that `row` can be longer than the number of categories, but not less.
  */
  pub fn get_item(&self, category: &str, row: &Vec<&'a str>) -> Result<&'a str, &'static str> {
    let index: usize = match self.categories.iter().position(|e| *e == category) {
      Some(val) => val,
      None => return Err("Invalid category (category not in categories vector).")
    };

    if row.len() < self.categories.len() {
      return Err("Invalid row vector (row's length is not valid).");
    }
    
    Ok(row[index])
  }
  
  /**
    Outputs a list of all values for a category.

    Returns `Ok` with a list of the values for the given category, or `Err` with a mesage if the category doesn't exist.
  */
  pub fn list_category(&self, category: &str) -> Result<Vec<&'a str>, &'static str> {
    let index: usize = match self.categories.iter().position(|e| *e == category) {
      Some(val) => val,
      None => return Err("Invalid category (category not in categories vector).")
    };
    
    let mut output: Vec<&'a str> = vec![];
    for row in &self.data {
      output.push(&row[index]);
    }
    
    Ok(output)
  }
  
  /**
    Lists all values for each given category.

    Returns `Ok` with a list of values for each category, or `Err` with the message (from `list_category`) if a category is not in the catagories for the `CSV` instacnce.
   
  */
  pub fn select_categories(&self, categories: Vec<&str>) -> Result<Vec<Vec<&'a str>>, &'static str> {
    let mut output: Vec<Vec<&'a str>> = vec![];
    for cat in categories {
      output.push(
        match self.list_category(cat) {
          Ok(val) => val,
          Err(msg) => return Err(msg)
        }
      );
    }
    
    Ok(output)
  }
  
  /**
    Reads each line of a string into the `CSV` instance.

    Loops through all lines in string, even if reading in the line would fail. If none of the lines would fail to be read, then all lines are read. Otherwise, no lines are read, and `Err` is returned.

    Returns `Ok` with the number of rows, or `Err` with a vector of indices where `read_line` would have failed.
  */
  pub fn read_str(&mut self, src: &'a str) -> Result<usize, Vec<usize>> {
      let lines: Vec<&str> = src.split('\n').collect();
      
      let mut lines_vec: Vec<&str> = vec![];
      let mut err_vec: Vec<usize> = vec![];
      let mut ok = true;
      
      for i in 0..lines.len() {
        let line = lines[i];
        let split_line: Vec<&str> = line.split(',').collect();
        
        if split_line.len() != self.categories.len() {
            ok = false;
            err_vec.push(i);
            continue;
        }
        
        lines_vec.push(line);
      };
      
      if !ok {
        return Err(err_vec);
      }
      
      for line in lines_vec {
        self.read_line(line).unwrap();
      }
      
      Ok(self.data.len())
  }
  
  /**
    Similar to `read_str`, but will read in every line until all lines are read or it finds an error. Then it will short circuit (hence "sc"). This can result in only some of the lines being read in.

    Returns `Ok` with number of rows, or `Err` with the index of the first line that failed to be read (via `read_line`).
  
  */
  pub fn read_str_sc(&mut self, src: &'a str) -> Result<usize, usize> {
    let lines: Vec<&str> = src.split('\n').collect();
    for i in 0..lines.len() {
      match self.read_line(lines[i]) {
        Err(_) => return Err(i),
        _ => ()
      }
    }
    
    Ok(self.data.len())
  }
}


impl<'a> fmt::Display for CSV<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut display = String::new();
    
    for category in &self.categories {
      display.push_str(category);
      display.push_str(",");
    }
    
    write!(f, "CSV categories: {}", &display[..display.len()-1])
  }
}