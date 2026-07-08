//! Runtime lifecycle lanes: launch → replacement → planning → activation → execution → host observation.

mod active;
mod activation;
#[path = "compat_modules.rs"]
mod compat_modules;
mod execution;
mod exports;
mod host_observation;
mod launch;
mod measurement;
mod planning;
pub mod replacement;
mod source_ingress;

/// Compatibility path for host adapter impl blocks.
pub mod host {
    pub use super::launch::host::WorthUiRuntimeHost;
    pub use super::launch::{WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial};
}

pub use compat_modules::*;
pub use exports::*;

#[cfg(test)]
mod tests;
