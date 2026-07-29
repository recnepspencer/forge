//! Bank-world semantic contract.
//!
//! Query execution, transport, fixture, and authentication mechanisms are not
//! owned here.

#![forbid(unsafe_code)]

pub mod accounting;
pub mod authorization;
pub mod estate;
pub mod model;
pub mod payments;
pub mod proposals;
pub mod reads;
pub mod schema;
