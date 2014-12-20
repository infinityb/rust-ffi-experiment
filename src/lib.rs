extern crate libc;

use libc::{c_int, c_uchar, c_uint, size_t};
use std::mem;

#[no_mangle]
pub extern "C" fn foo() -> c_uint {
    4
}

#[no_mangle]
pub extern "C" fn doubleit(x: c_uint) -> c_uint {
    2 * x
}

#[no_mangle]
pub extern "C" fn count_ones(vals: *mut c_uchar, len: size_t) -> c_int {
    if vals.is_null() {
        return 0;
    }

    let mut svals: &mut [u8] = unsafe {
        mem::transmute(std::raw::Slice {
            data: vals,
            len: len as uint, // ???
        })
    };

    let mut alloced = Vec::with_capacity(len as uint);

    let mut counter = 0;
    for (idx, val) in svals.iter_mut().enumerate() {
        if *val == 1 {
            alloced.push(idx);
            counter += 1;
            // replaces the 1s with their index mod 256, cuz mut.
            *val = (idx % 256) as u8;
        }
        
    }
    println!("alloced = {}", alloced);
    counter
}