//! Containment entity data shapes.
//!
//! DOMAIN: Body, Lump, Region, and Shell schemas
//! for the volumetric ownership hierarchy.

pub(crate) mod body;
pub(crate) mod lump;
pub(crate) mod region;
pub(crate) mod shell;

pub(crate) use body::BodyData;
pub(crate) use lump::LumpData;
pub(crate) use region::RegionData;
pub(crate) use shell::ShellData;
