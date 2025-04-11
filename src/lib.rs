#![no_std]

pub mod mffi;
pub use mffi::*;

pub mod panic;
pub use panic::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_rust_test() {
    unsafe {
        print_c_test();
    }
}
