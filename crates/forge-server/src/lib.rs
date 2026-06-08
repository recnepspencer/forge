//! `forge-server` owns the typed server facade and network bootstrap boundary.
//!
//! Milestone 1 establishes:
//!
//! - one facade-owned bootstrap path
//! - explicit surface-family registration
//! - validated runtime assembly before serving
//! - an internal transport boundary rather than framework-shaped public API

#![forbid(unsafe_code)]

mod config;
mod diagnostics;
pub mod facade;
mod registration;
mod runtime;
pub mod surfaces;
mod transport;

pub use config::{
    ForgeServerBindAddress, ForgeServerConfig, ForgeServerConfigBuilder, ForgeServerConfigError,
};
pub use diagnostics::ForgeServerCounterSnapshot;
pub use facade::{ForgeServer, ForgeServerBuildError, ForgeServerBuilder};
pub use registration::{
    ForgeServerSurfaceFamily, ForgeServerSurfaceInventory, ForgeServerSurfaceRegistration,
    ForgeServerSurfaceRegistryError,
};
