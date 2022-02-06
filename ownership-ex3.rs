//leaking resources.
//unsound behavior in safe rust code.
fn main() {
  let mut data = vec![Box::new(0); 4];
  
  {
    //drain method takes a mutable reference of the vector rendering it inaccessible.
    let mut data_drainer = data.drain(..);
    
    //take two elements and drop them
    let _ = data_drainer.next();
    let _ = data_drainer.next();
    
    //now, we just 'forget' about data_drainer without calling its destructor.
    //It still have two elements at this point.
    std::mem::forget(data_drainer);
  }
  //ERROR! basically, at this point, the first and second values of data were previously dropped. 
  //The data we are trying to read is now free'd memory!
  //println!("{}", data[0]);  //Uncomment this line.
}
