//! A lightweight kernel object used to solve the synchronization problem between threads
//!
//! # Example
//! ```
//! use rtt_rs2::semaphore;
//! use rtt_rs2::Arc;
//! use rtt_rs2::thread::Thread;
//!
//! let g_sem = Arc::new(semaphore::Semaphore::new().unwrap());
//!
//! let th1_sem = g_sem.clone();
//! let th = Thread::new().name("th").stack_size(8192).start(move ||{
//!     for _ in 0..10 {
//!         th1_sem.release();
//!    	    Thread::mdelay(100);
//!     }
//! });
//!
//! let th2_sem = g_sem.clone();
//! let th = Thread::new().name(rec).stack_size(8192).start(move ||{
//!     loop{
//!         th2_sem.take_wait_forever();
//!         print!("Rec Sem");
//!     }
//! });
//! ```

#![allow(dead_code)]

use crate::api::*;
use crate::{panic_on_atomic_context, RTTError};
use core::cell::UnsafeCell;

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

pub struct Semaphore(UnsafeCell<APIRawSem>);

impl Semaphore {
    pub fn new() -> Result<Self, RTTError> {
        semaphore_create("name")
            .ok_or(RTTError::OutOfMemory)
            .map(|m| Semaphore(UnsafeCell::new(m)))
    }

    pub fn new_with_name(name: &str) -> Result<Self, RTTError> {
        semaphore_create(name)
            .ok_or(RTTError::OutOfMemory)
            .map(|m| Semaphore(UnsafeCell::new(m)))
    }

    pub fn try_take(&self) -> Result<(), RTTError> {
        unsafe {
            let m = semaphore_try_take(*self.0.get());
            if !is_eok(m) {
                return Err(RTTError::SemaphoreTakeTimeout);
            }
            Ok(())
        }
    }

    pub fn take_wait_forever(&self) -> Result<(), RTTError> {
        panic_on_atomic_context("sem wait forever");
        let ret = unsafe { semaphore_take(*self.0.get(), -1) };

        if !is_eok(ret) {
            return Err(RTTError::SemaphoreTakeTimeout);
        }

        Ok(())
    }

    pub fn take(&self, max_wait: i32) -> Result<(), RTTError> {
        panic_on_atomic_context("sem take");
        let ret = unsafe { semaphore_take(*self.0.get(), max_wait) };

        if !is_eok(ret) {
            return Err(RTTError::SemaphoreTakeTimeout);
        }

        Ok(())
    }

    pub fn release(&self) {
        unsafe {
            semaphore_release(*self.0.get());
        }
    }

    fn drop(&mut self) {
        unsafe { semaphore_delete(*self.0.get()) }
    }
}
