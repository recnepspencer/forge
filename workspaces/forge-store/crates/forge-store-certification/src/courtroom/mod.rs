//! Courtroom modules aggregate scenario execution, harness, replay, and closeout surfaces.

pub mod blobs;
pub mod closeout;
pub(crate) mod cross_cutting;
pub(crate) mod durability;
pub(crate) mod foundational;
pub mod harness;
pub(crate) mod layout;
pub(crate) mod memory;
pub(crate) mod physical_integrity;
pub(crate) mod physical_isolation;
pub(crate) mod physical_substrate;
pub(crate) mod recovery;
pub mod replay;
pub mod scenario;
pub mod scheduling;
pub(crate) mod security;
pub(crate) mod source_tree;
