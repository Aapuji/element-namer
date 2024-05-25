use std::env;
use std::fs;
use std::process;

use element_namer::analyzer::{Tree, analyze};
use element_namer::table::ElementTable;

fn main() {
  let string = env::args().skip(1).next();

  if let None = string {
    eprintln!("Incorrect arguments: pass in the string you want to analyze.");
  }

  let table = ElementTable::from_csv("data.csv").expect("An error occured");
  let tree = Tree::empty();
  let output = analyze(string.unwrap().to_owned(), &table, &mut Box::new(tree));

  println!("{:#?}", output);
}