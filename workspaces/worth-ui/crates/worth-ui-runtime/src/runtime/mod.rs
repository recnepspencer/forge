//! Runtime lifecycle lanes: launch → replacement → planning → activation → execution → host observation.

mod activation;
mod active;
#[path = "compat_modules.rs"]
mod compat_modules;
mod execution;
pub(crate) mod exports;
mod host_observation;
mod launch;
mod measurement;
mod planning;
pub mod replacement;
mod source_ingress;

/// Compatibility path for host adapter impl blocks.
pub mod host {
    pub use super::launch::host::WorthUiRuntimeHost;
}

pub use compat_modules::*;
pub use exports::*;
pub(crate) use launch::WorthUiRuntimeHost;

#[cfg(test)]
pub(crate) use replacement::file_rust_replacement_parity;

#[cfg(test)]
pub(crate) mod tests;
