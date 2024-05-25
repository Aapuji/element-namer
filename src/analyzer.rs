use crate::table::ElementTable;

#[derive(Debug)]
pub enum Branch<T = Option<String>> {
  Left(T),
  Right(T)
}

pub fn analyze(str: String, table: &ElementTable) -> String {
  if str.len() == 0 {
    return todo!();
  }

  let data = table.data();

  // 1-letter test
  let mut target = &str[0..1];
  if let Some(i) = ElementTable::column_get_idx(&data.1, target.to_owned()) {
    let mut res = target.to_owned();
    res.push_str(&analyze(str[1..].to_owned(), table));
    
    return todo!();
  }

  if str.len() == 1 {
    return String::from("");
  }

  // 2-letter test
  target = &str[0..2];
  if let Some(i) = ElementTable::column_get_idx(&data.1, target.to_owned()) {
    let mut res = target.to_owned();
    res.push_str(&analyze(str[2..].to_owned(), table));

    return todo!();
  }

  String::from("")
}

pub fn make_ascii_titlecase(str: &mut String){
  if let Some(r) = str.get_mut(0..1) {
    r.make_ascii_uppercase()
  }
}