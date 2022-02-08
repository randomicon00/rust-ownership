/// Rust ownership example
/// Declare a String that allocates `Hello` in the heap. 
/// Its size is then unknown at compile time.
fn main() {
  //Get a mutable reference of String which data is located in the heap.
  let mut word = String::from("Hello");
  // Push str function appends a literal ("hardcoded strings") to our word String.
  word.push_str(" there!");
  // The print function will output `Hello there!`
  println!("{}", word);
}
