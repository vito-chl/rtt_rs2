//! RT-THREAD & RUST
//!
//! A simple and easy-to-use system support library
//! that provides basic functions and FS, NET and DEVICE.
//!
//! You can use this library on embedded devices that support rt-thread


#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), cfg(not(test)))]

#![feature(alloc_error_handler)]
#![feature(allow_internal_unstable)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(linkage)]
#![allow(dead_code)]

mod puts;

pub extern crate alloc;

cfg_if::cfg_if! {

    if #[cfg(not(feature = "std"))] {
        #[doc = "Alloc by rt-thread"]
        #[global_allocator]
        static GLOBAL: malloc::RttAlloc = malloc::RttAlloc;
        pub mod api;
        mod bind;

        pub mod malloc;
        pub mod mutex;
        pub mod out;
        pub mod param;
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

        fn panic_atomic_context(s: &str) {
            use crate::api::is_irq_context;
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
    }
}
