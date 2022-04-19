/// A short example about how to create a static reference off a struct.
/// To do so, we get a raw pointer from a Box, which frees us up from 
/// the borrow checker's rules.  
#![allow(dead_code)]

#[derive(Debug)]
pub struct MyStruct {
    a: u8,
    b: String,
    c: bool,
}

fn get_static_ref() -> &'static MyStruct {
    let my_struct = Box::new(MyStruct {
        a: 1u8,
        b: "foo".to_owned(),
        c: true,
    });
   
    // Get a raw pointer and disable the borrow checker
    // SAFETY: Accessing the following value through a static immutable reference which
    // ensures that the value behind the reference will never be null or undefined.
    let ptr = Box::into_raw(my_struct);
    
    
    // Dereference the raw pointer and coerce the reference
    // to the static lifetime.
    unsafe { &*ptr }
}

fn main() {
    let my_static_ref = get_static_ref();
    
    println!("{:?}", my_static_ref);
    
} // <--- the static reference gets dropped at the end of the main thread program execution
