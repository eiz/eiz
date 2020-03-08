#![cfg_attr(all(not(test), not(use_std)), no_std)]
#![feature(alloc_layout_extra, maybe_uninit_extra)]
extern crate alloc;

#[cfg(feature = "rt_queue")]
pub mod rt_queue;
