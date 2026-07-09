//! Capability registry lane — descriptor → freeze snapshot → diagnostics → support inventory.

pub mod descriptor;
pub mod diagnostics;
pub mod snapshot;
pub mod support;

pub use descriptor::*;
pub use diagnostics::*;
pub use snapshot::*;
pub use support::*;
