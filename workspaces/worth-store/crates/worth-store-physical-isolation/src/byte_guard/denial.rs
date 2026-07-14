use super::PhysicalByteGuardScope;
use crate::PhysicalByteGuardScopeKind;
use worth_store_buffer_pool::ResidentFrameDenial;
use worth_store_physical_format::PhysicalGenerationOwner;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalByteGuardDenial {
    ReachabilityLeaseIsNotByteGuard,
    GuardScopeMismatch {
        expected: PhysicalByteGuardScope,
        observed: PhysicalByteGuardScope,
    },
    GuardScopeKindMismatch {
        expected: PhysicalByteGuardScopeKind,
        observed: PhysicalByteGuardScopeKind,
    },
    ByteProvenanceMismatch {
        expected: PhysicalGenerationOwner,
        observed: PhysicalGenerationOwner,
    },
    ResidentFrameDenied(ResidentFrameDenial),
    UnsupportedGuardScope(PhysicalByteGuardScope),
}

impl From<ResidentFrameDenial> for PhysicalByteGuardDenial {
    fn from(denial: ResidentFrameDenial) -> Self {
        Self::ResidentFrameDenied(denial)
    }
}
