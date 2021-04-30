#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/dog.rs"));

fn main() {
    let name = "Inu";
    let name = std::ffi::CString::new(name).unwrap();
    let name_pointer = name.as_ptr();
    unsafe {
        let mut dog = Dog::new(name_pointer);
        dog.walk();
        dog.stop();
        dog.destruct();
    }
}
