use crate::BackendCapabilityAdmissionDenial;

use super::AccessPolicyCounterSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessPolicyDenialKind {
    MissingPhysicalReference,
    MissingSecurityScope,
    MissingBufferLifecycle,
    MissingPageCachePolicy,
    DirectIoAlignmentRequired,
    MmapFaultPostureUnsupported,
    MixedModeCoherenceRequired,
    InvalidMixedAccessTransition,
    DirtyMmapPageBlocksDirectIo,
    BackendCapabilityDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyDenial {
    kind: AccessPolicyDenialKind,
    counters: AccessPolicyCounterSnapshot,
    backend_denial: Option<BackendCapabilityAdmissionDenial>,
}

impl AccessPolicyDenial {
    pub(crate) const fn new(
        kind: AccessPolicyDenialKind,
        counters: AccessPolicyCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            counters,
            backend_denial: None,
        }
    }

    pub(crate) const fn from_backend(
        backend_denial: BackendCapabilityAdmissionDenial,
        counters: AccessPolicyCounterSnapshot,
    ) -> Self {
        Self {
            kind: AccessPolicyDenialKind::BackendCapabilityDenied,
            counters,
            backend_denial: Some(backend_denial),
        }
    }

    pub const fn kind(self) -> AccessPolicyDenialKind {
        self.kind
    }
    pub const fn counters(self) -> AccessPolicyCounterSnapshot {
        self.counters
    }
    pub const fn backend_denial(self) -> Option<BackendCapabilityAdmissionDenial> {
        self.backend_denial
    }
}
