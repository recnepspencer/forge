#![forbid(unsafe_code)]

pub mod artifact_lifecycle;
mod authority_progression;
pub mod ci;
pub mod classification;
pub mod cli;
pub mod closeout;
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
