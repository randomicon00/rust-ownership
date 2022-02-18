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
    
    let ptr = Box::into_raw(my_struct);
    
    unsafe { &*ptr }
}

fn main() {
    let my_static_ref = get_static_ref();
    println!("{:?}", my_static_ref);
}
