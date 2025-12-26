#![allow(clippy::cargo_common_metadata)]
#![allow(unused_variables)]
#![doc = include_str!("../../../README.md")]

mod rt;

#[cfg(test)]
mod tests;

pub use crate::rt::{Runtime, RuntimeError, RuntimeResult, RuntimeReturnValues};
