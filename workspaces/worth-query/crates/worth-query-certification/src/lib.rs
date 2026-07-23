//! Cold, reusable certification support for runtime-installed Query domains.
//!
//! Ordinary applications use `worth-query-host`. Certification packages add
//! this crate to compare admitted providers and exercise Query-owned hostile
//! scenarios without gaining construction authority.

#![forbid(unsafe_code)]

mod evidence;
pub mod facade;
mod oracle;
mod scenario;
