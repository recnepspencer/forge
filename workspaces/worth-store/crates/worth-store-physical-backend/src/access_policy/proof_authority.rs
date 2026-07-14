use crate::{AdmittedBackendCapabilityWitness, PhysicalReference};
use worth_store_physical_format::{PhysicalAlignmentClass, PhysicalAlignmentSite};

use super::coherence::{MixedInvalidationPosture, MixedWritebackPosture};
use super::{
    AccessPolicyBufferLifecycle, AccessPolicySecurityScope, DirectIoAlignmentRequirement,
    MixedAccessCoherenceBasis, MixedAccessTransition, MmapFaultHandling, MmapFaultPosture,
    MmapPunchHolePosture, MmapTruncatePosture, MmapVisibilityPosture, MmapWritebackPosture,
    PageCachePolicyProof, StoreAccessMode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreAccessPolicyProofAuthority {
    backend: AdmittedBackendCapabilityWitness,
}

impl StoreAccessPolicyProofAuthority {
    pub const fn for_admitted_backend(backend: &AdmittedBackendCapabilityWitness) -> Self {
        Self { backend: *backend }
    }

    pub fn page_cache_policy(self) -> Option<PageCachePolicyProof> {
        self.backend
            .media_assumptions()
            .supports_page_cache_policy()
            .then(PageCachePolicyProof::store_admitted_visibility)
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn direct_io_unaligned_for_denial(
        self,
        reference: PhysicalReference,
        lifecycle: AccessPolicyBufferLifecycle,
    ) -> DirectIoAlignmentRequirement {
        DirectIoAlignmentRequirement::unaligned(reference, lifecycle)
    }

    pub fn direct_io_page_and_sector_aligned(
        self,
        reference: PhysicalReference,
        lifecycle: AccessPolicyBufferLifecycle,
        byte_length: u32,
        page_alignment: PhysicalAlignmentClass,
        extent_alignment: PhysicalAlignmentClass,
    ) -> Option<DirectIoAlignmentRequirement> {
        (self
            .backend
            .media_assumptions()
            .supports_direct_io_alignment()
            && page_alignment.site() == PhysicalAlignmentSite::PageStart
            && page_alignment.bytes() >= 4096
            && extent_alignment.site() == PhysicalAlignmentSite::ExtentStart
            && extent_alignment.bytes() >= 4096
            && byte_length > 0
            && byte_length % page_alignment.bytes() as u32 == 0)
            .then(|| {
                DirectIoAlignmentRequirement::page_and_sector(
                    reference,
                    lifecycle,
                    byte_length,
                    page_alignment,
                    extent_alignment,
                )
            })
    }

    pub fn mmap_posture(self) -> Option<MmapFaultPosture> {
        self.backend
            .media_assumptions()
            .supports_admitted_mmap_access_policy()
            .then(|| {
                MmapFaultPosture::new(
                    MmapFaultHandling::FaultsSurfaceAsTypedViolation,
                    MmapWritebackPosture::StoreTrackedDirtyWriteback,
                    MmapVisibilityPosture::SharedVisibilityAdmitted,
                    MmapTruncatePosture::TypedFaultOnTruncate,
                    MmapPunchHolePosture::TypedFaultOnPunchHole,
                )
            })
    }

    pub fn mixed_coherence(
        self,
        transition: MixedAccessTransition,
        reference: PhysicalReference,
        security_scope: AccessPolicySecurityScope,
    ) -> Option<MixedAccessCoherenceBasis> {
        (self
            .backend
            .media_assumptions()
            .supports_mixed_access_coherence()
            && transition.has_only_physical_participants())
        .then(|| {
            MixedAccessCoherenceBasis::new(
                transition,
                reference,
                security_scope,
                mixed_invalidation_posture(transition),
                mixed_writeback_posture(transition),
            )
        })
    }
}

const fn mixed_invalidation_posture(transition: MixedAccessTransition) -> MixedInvalidationPosture {
    if transition.involves(StoreAccessMode::Mmap) {
        MixedInvalidationPosture::MmapCleanSharedVisibility
    } else {
        MixedInvalidationPosture::PageCacheInvalidated
    }
}

const fn mixed_writeback_posture(transition: MixedAccessTransition) -> MixedWritebackPosture {
    if transition.involves(StoreAccessMode::Mmap) {
        MixedWritebackPosture::DirtyWritebackSequencedByStore
    } else {
        MixedWritebackPosture::NoDirtyWritebackRace
    }
}
