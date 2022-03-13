/// Rust ownership example
/// Declare a String that allocates `Hello` in the heap. 
/// Its size is then unknown at compile time.
fn main() {
  // Create a mutable String which data is located in the heap.
  let mut word = String::from("Hello,");
  
  // Create a mutable Vector which data is located in the heap. 
  let mut numbers = vec![10u8, 20, 30];
  
  // Push str function appends a literal ("hardcoded strings") to our word of type String.
  word.push_str(" there!");
  
  // Append a number at the end of the array. 
  numbers.push(40);
  
  println!("{}", word);       // The print function will output `Hello there!`
  
  println!("{:?}", numbers);  // The print function will output `[10, 20, 30, 40]`
}
