use crate::table::ElementTable;

/** A tree to hold the different possible paths that the analyzer could take.
  * `left` is the version when an element matches just the first letter.
  * `right` is the version when an element matches both the first and second letters.
  * `index` holds the index of the element in the table (starts at 0; 1 less than atomic number).
  * The value of `index` for the very topmost node in the entire tree doesn't matter, as the strings start from its children. 
 */
#[derive(Debug, Clone)]
pub struct Tree {
  index: usize,
  left: Option<Box<Tree>>,
  right: Option<Box<Tree>>
}

impl Tree {
  pub fn empty() -> Self {
    Tree { 
      index: 0, 
      left: None,
      right: None
    }
  }

  pub fn new(index: usize) -> Self {
    Tree {
      index,
      left: None,
      right: None
    }
  }

  pub fn traverse(&self) -> Vec<usize> {
    let mut indices: Vec<Vec<usize>> = vec![];
    let mut current: &Tree;

    // Left branches (first element is 1 letter)
    if self.left.is_some() {

    }

    // Right branches (first element is 2 letters)
    if self.right.is_some() {
      let mut branch: Vec<usize> = vec![];
      
      current = self.right.as_ref().unwrap();
      


    }

    todo!()
  }

  /*
  /** Given the start node, `current`, this traverses down the tree using the `left` fields, mutating the `branch` vector with the indices from the nodes. It returns the new `current` value. */
  fn traverse_leftwards_branch<'a>(&self, mut current: &'a Tree, branch: &mut Vec<usize>) -> &'a Tree {
    while current.left.is_some() {
      branch.push(current.index);
      current = current.left.as_ref().unwrap();
    }

    current
  }
  */


}

pub fn analyze(str: String, table: &ElementTable, tree: &mut Box<Tree>) -> Option<Tree> {
  // Reached end of string in current branch
  if str.len() == 0 {
    return None;
  }

  let data = table.data();

  // 1-letter test
  let mut target = &str[0..1];
  if let Some(i) = ElementTable::column_get_idx(&data.1, target.to_owned()) {
    tree.left = Some(Box::new(Tree::new(i)));
    analyze(str[1..].to_owned(), table, &mut tree.left.as_mut().unwrap());
  }

  if str.len() == 1 {
    tree.right = None;
    return Some(*tree.clone()); // Gets the `Tree` out of the `Box` and into the `Some`
  }

  // 2-letter test
  target = &str[0..2];
  if let Some(i) = ElementTable::column_get_idx(&data.1, target.to_owned()) {
    tree.right = Some(Box::new(Tree::new(i)));
    analyze(str[2..].to_owned(), table, tree.right.as_mut().unwrap());
  }

  return Some(*tree.clone()); // Gets the `Tree` out of the `Box` and into the `Some`
}

pub fn make_ascii_titlecase(str: &mut String){
  if let Some(r) = str.get_mut(0..1) {
    r.make_ascii_uppercase()
  }
}