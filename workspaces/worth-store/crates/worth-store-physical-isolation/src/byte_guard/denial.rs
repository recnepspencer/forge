use super::PhysicalByteGuardScope;
use crate::PhysicalByteGuardScopeKind;
#[cfg(feature = "legacy-certification-models")]
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
    #[cfg(feature = "legacy-certification-models")]
    ResidentFrameDenied(ResidentFrameDenial),
    UnsupportedGuardScope(PhysicalByteGuardScope),
}

#[cfg(feature = "legacy-certification-models")]
impl From<ResidentFrameDenial> for PhysicalByteGuardDenial {
    fn from(denial: ResidentFrameDenial) -> Self {
        Self::ResidentFrameDenied(denial)
    }
}
