//! Arena storage infrastructure.
//!
//! DOMAIN: Generational slot allocator and the central TopologyArena container.

pub(crate) mod arena;
pub(crate) mod cache_runtime;
pub(crate) mod sidecar_accessors;
pub(crate) mod slot;

pub(crate) use arena::TopologyArena;
