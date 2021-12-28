//! Passing information between threads

use crate::api::*;
use crate::{Box, RTTError};
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem;

unsafe impl<T> Send for Queue<T> where T: Send {}
unsafe impl<T> Sync for Queue<T> where T: Send {}

#[derive(Debug)]
pub struct Queue<T> {
    queue: APIRawQueue,
    /* only for store item type */
    item_type: PhantomData<UnsafeCell<T>>,
}

impl<T> Queue<T> {
    pub fn new(max_size: usize) -> Result<Queue<T>, RTTError> {
        queue_create("Unnamed", max_size as _, Self::mem_size() as _)
            .ok_or(RTTError::OutOfMemory)
            .map(|m| Queue {
                queue: m,
                item_type: PhantomData,
            })
    }

    pub fn new_with_name(name: &str, max_size: usize) -> Result<Queue<T>, RTTError> {
        queue_create(name, max_size as _, Self::mem_size() as _)
            .ok_or(RTTError::OutOfMemory)
            .map(|m| Queue {
                queue: m,
                item_type: PhantomData,
            })
    }

    #[inline]
    pub const fn mem_size() -> usize {
        mem::size_of::<*mut T>()
    }

    pub fn send(&self, item: T) -> Result<(), RTTError> {
        Self::send_wait(&self, item, 0)
    }

    pub fn send_wait(&self, item: T, max_wait: i32) -> Result<(), RTTError> {
        let s = Box::new(item);
        let s = Box::new(s);
        let s = Box::into_raw(s);

        let ret = queue_send_wait(
            self.queue,
            &s as *const _ as *const c_void,
            Self::mem_size() as _,
            max_wait,
        );

        return if !is_eok(ret) {
            Err(RTTError::QueueSendTimeout)
        } else {
            Ok(())
        };
    }

    pub fn receive(&self, max_wait: i32) -> Result<T, RTTError> {
        let mut ptr = 0 as *mut Box<T>;
        let ret = queue_receive_wait(
            self.queue,
            &mut ptr as *mut _ as *mut c_void,
            Self::mem_size() as _,
            max_wait,
        );
        return if is_eok(ret) {
            Ok(unsafe {
                let y = Box::from_raw(ptr);
                **y
            })
        } else {
            Err(RTTError::QueueReceiveTimeout)
        };
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        queue_delete(self.queue);
    }
}
