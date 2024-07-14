use open;

use std::{ffi::CStr, os::raw::{c_char, c_int}};


#[no_mangle]
pub extern "C" fn simple_open(url: *const c_char ) -> c_int{

    if url.is_null() {
        return -1;
    }

    print!("rs: simple_open(..)\n");

    // Convert the raw C string to a Rust string
    let c_str = unsafe { CStr::from_ptr(url) };
    let url_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };


    let path = url_str; //"http://rust-lang.org";

    match open::that(path) {
        Ok(()) => println!("Opened '{}' successfully.", path),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }

    
    return 0;
}

