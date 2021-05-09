#![feature(once_cell)]
#![feature(try_trait)] // 1.53.0-nightly (2021-04-01 d474075a8f28ae9a410e)

pub mod cache;
mod compiler;
pub mod err;
pub mod identity;
pub mod policy;
pub mod storage;
pub mod utils;

pub use utils::glob_to_regex;
