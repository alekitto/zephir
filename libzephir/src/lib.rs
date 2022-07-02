#![feature(hash_drain_filter)]

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

pub fn initialize_libzephir() {}
