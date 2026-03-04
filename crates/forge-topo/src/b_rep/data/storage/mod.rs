//! Arena storage infrastructure.
//!
//! DOMAIN: Generational slot allocator and the central TopologyArena container.

pub(crate) mod slot;
pub(crate) mod arena;
pub(crate) mod sidecar_accessors;

pub(crate) use arena::TopologyArena;
