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

  /// Outputs all the possible paths through the tree that don't have sentinel values.
  pub fn traverse(&self) -> Vec<Vec<usize>> {
    let mut all_paths: Vec<Vec<usize>> = vec![];
    let mut current_path: Vec<usize> = vec![];

    // Left subtree (first element's symbol is only 1 letter)
    Tree::depth_first_search(rem_box_ref(&self.left), &mut current_path, &mut all_paths);

    // Right subtree (first element's symbol is 2 letters)
    Tree::depth_first_search(rem_box_ref(&self.right), &mut current_path, &mut all_paths);

    all_paths
  }

  /// Searches through the tree and adds each path without a sentinel value to `all_paths`. 
  fn depth_first_search(node: Option<&Self>, path: &mut Vec<usize>, all_paths: &mut Vec<Vec<usize>>) {
    if node.is_none() {
      return;
    }
  
    let tree = node.unwrap();

    // Throws away a path if it finds the sentinel value that means the string isn't finished
    if tree.index == usize::MAX {
      return;
    }
  
    path.push(tree.index);
  
    if tree.left.is_none() && tree.right.is_none() {
      all_paths.push(path.clone());
    } else {
      Tree::depth_first_search(rem_box_ref(&tree.left), path, all_paths);
      Tree::depth_first_search(rem_box_ref(&tree.right), path, all_paths);
    }
  
    path.pop();
  }
}

/** Takes a given string in lowercase and outputs a tree that holds the paths of possible elements.
 
 Nodes on the left are elements with one letter and those on the right have 2 letters.

 When the analyzer comes to a string and is not able to find a matching element symbol, it changes the index to be `usize::MAX` (denoted as `$MAX` in the diagram) as a sentinel value that the traversal algorithm can use to throw away the path.
  
  For example, the string `"genius"` would generate the following tree:
  ```txt
        #
         \
          ge
         /  \ 
        n    ni
       /       \
      i         u
     /           \
    u             s
   / 
  s               
  ```
  And the string `"hina"` would generate the following tree:
  ```txt
           #
          /
         h
        / \ 
       i   in
      / \    \
     n   na $MAX
    / 
  $MAX
  ```
*/
pub fn analyze(str: String, table: &ElementTable, tree: &mut Box<Tree>) -> Option<Tree> {
  // Reached end of string in current branch
  if str.len() == 0 {
    return None;
  }

  let data = table.data();
  let mut found_valid_element = false;

  // 1-letter test
  let mut target = &str[0..1];
  if let Some(i) = ElementTable::column_get_idx(&data.1, target.to_owned()) {
    found_valid_element = true;
    tree.left = Some(Box::new(Tree::new(i)));
    analyze(str[1..].to_owned(), table, &mut tree.left.as_mut().unwrap());
  }

  if str.len() == 1 {
    tree.right = None;

    // 1 letter left in string and it is not a valid element
    // Enters a leaf node that has an index of `usize::MAX` as a sentinel value
    // When the traverser sees this value, it will throw away the current path, 
    // instead of adding it 
    if !found_valid_element {
      tree.index = usize::MAX;
    }

    // Gets the `Tree` out of the `Box` and into the `Some`
    return Some(*tree.clone()); 
  }

  // 2-letter test
  target = &str[0..2];
  if let Some(i) = ElementTable::column_get_idx(&data.1, target.to_owned()) {
    found_valid_element = true;
    tree.right = Some(Box::new(Tree::new(i)));
    analyze(str[2..].to_owned(), table, tree.right.as_mut().unwrap());
  }

  if !found_valid_element {
    tree.index = usize::MAX;
  }

  return Some(*tree.clone()); // Gets the `Tree` out of the `Box` and into the `Some`
}

pub fn make_ascii_titlecase(str: &mut String){
  if let Some(r) = str.get_mut(0..1) {
    r.make_ascii_uppercase()
  }
}

/// Given a reference to an `Option<Box<T>>`, it replaces the `Box` with a `&`.
fn rem_box_ref<'a, T>(option: &'a Option<Box<T>>) -> Option<&'a T> {
  option.as_ref().map(|b| b.as_ref())
}