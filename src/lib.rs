#![cfg_attr(all(not(test), not(feature = "use_std")), no_std)]
#![cfg_attr(feature = "rt_queue", feature(alloc_layout_extra, maybe_uninit_extra))]
extern crate alloc;

#[cfg(feature = "com")]
pub mod com;

#[cfg(feature = "decklink")]
pub mod decklink;

#[cfg(feature = "rt_queue")]
pub mod rt_queue;

#[cfg(feature = "nvenc")]
pub mod nvenc;
