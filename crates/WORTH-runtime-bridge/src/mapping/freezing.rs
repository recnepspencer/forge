use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::mapping::lookup::BridgeMappingLookupKey;
use crate::mapping::registration::{
    AspectKeySelector, BridgeFrozenMappingRegistrationIdentity, BridgeMappingRegistration,
    CoarseRoutingMode, MappingSelector, TruthPatchScope, TruthPatchTargetSelector,
};
use crate::mapping::widening::BridgeMappingWideningClass;

mod registry;
mod validation;

pub use registry::*;

#[cfg(test)]
mod tests;
