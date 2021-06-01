#![cfg_attr(all(not(test), not(use_std)), no_std)]
#![cfg_attr(feature = "rt_queue", feature(alloc_layout_extra, maybe_uninit_extra))]
extern crate alloc;

#[cfg(feature = "rt_queue")]
pub mod rt_queue;

#[cfg(feature = "nvenc")]
pub mod nvenc;
