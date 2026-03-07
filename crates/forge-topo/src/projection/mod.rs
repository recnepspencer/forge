pub mod data;
pub mod logic;
pub mod presentation;
#[cfg(test)]
pub mod testing;

pub use facade::*;

mod facade;
