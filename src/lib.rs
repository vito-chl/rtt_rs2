//! RT-THREAD & RUST
//!
//! A simple and easy-to-use system support library
//! that provides basic functions and FS, NET and DEVICE.
//!
//! You can use this library on embedded devices that support rt-thread
//!
//! You can specify a function for “ C ” to call
//!
//! # Example
//! ```
//! #![no_std]
//!
//! use rtt_rs2::*;
//! entry!(main);
//! fn main() { /*.....*/ }
//! ```

#![no_std]
#![feature(alloc_error_handler)]
#![feature(allow_internal_unstable)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(linkage)]
#![cfg(not(test))]
#![allow(dead_code)]

#[doc = "alloc by rt-thread"]
#[global_allocator]
static GLOBAL: malloc::RttAlloc = malloc::RttAlloc;

pub extern crate alloc;
pub mod api;
mod bind;
pub mod malloc;

pub mod mutex;
pub mod out;
pub mod queue;
pub mod semaphore;
pub mod thread;

mod prelude;
pub use prelude::v1::*;

// TODO: review this enum
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RTTError {
    ThreadStartupErr,
    MutexTakeTimeout,
    SemaphoreTakeTimeout,
    QueueSendTimeout,
    QueueReceiveTimeout,
    OutOfMemory,

    DeviceNotFound,
    DeviceOpenFailed,
    DeviceCloseFailed,
    DeviceReadFailed,
    DeviceWriteFailed,
    DeviceTransFailed,
    DeviceConfigFailed,
    DeviceSetRxCallBackFailed,
    DeviceSetTxCallBackFailed,

    FuncUnDefine,
}

pub type RTResult<T> = Result<T, RTTError>;

/// This macro is used to indicate the entry function of the system
/// Function `rust_main` will be called by rt-thread c code
///
/// # Example
/// ```
/// use rtt_rs2::*;
///
/// /* rt-thread will call function main */
/// entry!(main);
///
/// fn main(){
///     /* ..... */
/// }
/// ```
///
#[macro_export]
macro_rules! entry {
    ($func: ident) => {
        #[export(name = "rust_main")]
        fn __rust_main() -> usize {
            $func();
            0
        }
    };
}

use crate::api::is_irq_context;

fn panic_atomic_context(s: &str) {
    if is_irq_context() {
        panic!("{}", s);
    }
}

#[panic_handler]
#[inline(never)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("{:}", info);
    __rust_panic()
}

#[linkage = "weak"]
#[no_mangle]
fn __rust_panic() -> ! {
    loop {}
}
