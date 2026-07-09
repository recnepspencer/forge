//! Named harness topology for Store certification and replay scenarios.
//!
//! `production_facade` contains helpers that exercise production-owned
//! capabilities through their public lifecycle. `test_authority` contains the
//! synthetic courtroom-only witnesses and shortcut attempts that exist only to
//! falsify production topology.

pub mod fixtures;
mod milestone;
pub mod physical_reference;
pub mod physical_simulation;
pub mod production_facade;
pub mod test_authority;

pub use production_facade::*;
