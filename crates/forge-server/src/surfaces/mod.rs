mod capabilities;
mod facade;
mod root;

pub mod binary;
pub mod compat_http;
pub mod forge_native;
pub mod integration;
pub mod lease;
pub mod sync;

pub use capabilities::ForgeServerSurfaceCapabilities;
pub use facade::ForgeServerSurfacesFacade;
pub use root::{
    ForgeServerSurfaceFamilyMarker, ForgeServerSurfaceRoot, ForgeServerTypedSurfaceRoot,
};

pub use binary::{BinarySurface, BinarySurfaceRoot};
pub use compat_http::{CompatHttpSurface, CompatHttpSurfaceRoot};
pub use forge_native::{ForgeNativeSurface, ForgeNativeSurfaceRoot};
pub use integration::{IntegrationSurface, IntegrationSurfaceRoot};
pub use lease::{LeaseSurface, LeaseSurfaceRoot};
pub use sync::{SyncSurface, SyncSurfaceRoot};
