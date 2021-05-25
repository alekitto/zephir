#![feature(once_cell)]
#![feature(hash_drain_filter)]
#![feature(shrink_to)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

pub mod cache;
mod compiler;
pub mod err;
pub mod identity;
pub mod policy;
pub mod storage;
pub mod utils;
pub use utils::glob_to_regex;

use crate::policy::condition::initialize_v8_platform;

pub fn initialize_libzephir() {
    initialize_v8_platform().expect("Failed to initialize V8");
}
