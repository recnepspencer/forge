#![forbid(unsafe_code)]

mod authority_progression;
pub mod ci;
pub mod classification;
pub mod cli;
pub mod discovery;
pub mod evidence;
pub mod execution;
pub mod preservation;
pub mod selection;
pub mod structural_preflight;

pub use authority_progression::{
    ClassifiedProofInventory, DiscoveredTestSurface, ValidatedProofInventory,
};

#[cfg(test)]
mod tests;
