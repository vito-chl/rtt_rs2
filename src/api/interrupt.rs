// Interrupt
use crate::bind::*;

pub fn no_irq<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let level;
    let out;
    unsafe {
        level = rt_hw_interrupt_disable();
    }
    out = f();
    unsafe {
        rt_hw_interrupt_enable(level);
    }
    out
}

#[derive(Debug, Copy, Clone)]
pub struct InterruptFlag(rt_base_t);

pub const INTERRUPT_FLAG_INIT: InterruptFlag = InterruptFlag { 0: 0 as _ };

pub fn interrupt_disable() -> InterruptFlag {
    unsafe { InterruptFlag(rt_hw_interrupt_disable()) }
}

pub fn interrupt_enable(f: InterruptFlag) {
    unsafe {
        rt_hw_interrupt_enable(f.0);
    }
}

pub fn interrupt_enter() {
    unsafe {
        rt_interrupt_enter();
    }
}

pub fn interrupt_leave() {
    unsafe {
        rt_interrupt_leave();
    }
}

pub fn is_irq_context() -> bool {
    unsafe { rt_interrupt_get_nest() != 0 }
}
