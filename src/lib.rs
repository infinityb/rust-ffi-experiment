extern crate libc;
use libc::c_int;
use std::mem;

#[no_mangle]
pub extern "C" fn foo() -> uint {
    4
}

#[no_mangle]
pub extern "C" fn doubleit(x: uint) -> uint {
    2 * x
}

#[no_mangle]
pub extern "C" fn count_ones(vals: *mut u8, len: uint) -> c_int {
	if vals.is_null() {
		return 0;
	}

	let mut svals: &mut [u8] = unsafe {
		mem::transmute(std::raw::Slice {
			data: vals,
			len: len
		})
	};

	let mut alloced = Vec::with_capacity(len);

	let mut counter = 0;
	for (idx, val) in svals.iter_mut().enumerate() {
		if *val == 1 {
			alloced.push(idx);
			counter += 1;
			// replaces the 1s with their index mod 256
			*val = (idx % 256) as u8;
		}
		
	}
	println!("alloced = {}", alloced);
    counter
}