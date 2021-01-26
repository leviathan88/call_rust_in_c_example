use libc::{c_char, c_int};
use std::ffi::CStr;

mod interleave;
use interleave::Interleave;

#[no_mangle]
pub extern "C" fn add(a: *const c_int, b: *const c_int, result: *mut c_int) -> c_int {
    unsafe {
        *result = domain_logic::add(*a, *b);
    }
    0
}

#[no_mangle]
pub extern "C" fn solve_interleave(
    a: *const c_char,
    b: *const c_char,
    c: *const c_char,
    result: *mut bool,
) -> c_int {
    unsafe {
        let s1 = CStr::from_ptr(a);
        let s2 = CStr::from_ptr(b);
        let s3 = CStr::from_ptr(c);
        *result = domain_logic::solve_interleave(
            s1.to_str().unwrap(),
            s2.to_str().unwrap(),
            s3.to_str().unwrap(),
        );
    }
    0
}

mod domain_logic {
    use super::*;

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn solve_interleave(s1: &str, s2: &str, s3: &str) -> bool {
        let interleave = Interleave::new(&s1, &s2, &s3);
        interleave.solve()
    }
}
