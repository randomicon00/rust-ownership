fn main() {
   let mut mutable_vector = vec![1, 2, 3, 4, 5, 6];
   let reference_to_mutable_vector = &mut mutable_vector //&mut `mutable_vector` borrow starts here
   // code..
   // more code..
   
} //  <---- &mut `mutable_vector` borrow goes out of scope here
