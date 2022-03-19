/// Rust has no garbage collector.
/// It uses the RIIA principle which is the acronym for Resource Acquisition Is Initialization 
/// A variable is cleaned up once it goes out of scope. 
fn main() {
   let mut mutable_vector = vec![1, 2, 3, 4, 5, 6];
   
   let reference_to_mutable_vector = &mut mutable_vector //&mut `mutable_vector` borrow starts here
   
} //  <---- &mut `mutable_vector` borrow goes out of scope here
