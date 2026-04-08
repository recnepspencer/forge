use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::mapping::fallback::BridgeMappingFallbackClass;
use crate::mapping::lookup::BridgeMappingLookupKey;
use crate::mapping::registration::{BridgeMappingRegistration, MappingSelector};

mod registry;
mod validation;

pub use registry::*;

#[cfg(test)]
mod tests;
