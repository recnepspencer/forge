//! Arena storage infrastructure.
//!
//! DOMAIN: Generational slot allocator and the central TopologyArena container.

pub(crate) mod slot;
pub(crate) mod arena;

pub(crate) use arena::TopologyArena;
