use std::env;

use element_namer::analyzer::{analyze, get_results, Tree};
use element_namer::table::ElementTable;

fn main() {
  let string = {
    let mut ss = String::new();
    match env::args().skip(1).next() {
      None => eprintln!("Incorrect arguments: pass in the string you want to analyze"),
      Some(s) => ss = s
    }

    ss
  };

  let table = ElementTable::from_csv("data.csv").expect("An error occured");
  
  let empty = Tree::empty();
  let output = analyze(string.to_owned(), &table, &mut Box::new(empty.clone()));
  
  if output.is_none() {
    println!("No valid transcription for {:}", string);
  } else {
    let tree = output.unwrap();

    let indices = tree.traverse();
    println!("{:#?}", &indices);

    let results = get_results(indices, &table);
    println!("{:#.3?}", &results);
  }  
}