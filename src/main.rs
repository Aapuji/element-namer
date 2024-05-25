use std::env;
use std::process;

use element_namer::table::ElementTable;

fn main() {
  let args = env::args();

  if args.len() != 2 {
    eprintln!("Incorrect arguments: pass in the string you wantt o analyze.");
  }

  let table = ElementTable::from_csv("./data.csv").expect("An error occured");
  
}