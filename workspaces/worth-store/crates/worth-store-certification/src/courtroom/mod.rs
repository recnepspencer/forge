//! Courtroom modules aggregate direct scenario execution, harness, and replay surfaces.

pub mod blobs;
pub(crate) mod durability;
pub(crate) mod foundational;
pub mod harness;
pub(crate) mod layout;
pub(crate) mod memory;
#[cfg(test)]
pub(crate) mod physical_integrity;
pub(crate) mod physical_substrate;
pub mod protocol_models;
pub(crate) mod recovery;
pub mod scheduling;
#[cfg(test)]
pub(crate) mod security;
