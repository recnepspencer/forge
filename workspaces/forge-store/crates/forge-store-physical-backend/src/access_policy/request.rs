use crate::{CapabilityEvidenceClass, PhysicalReference};

use super::{
    AccessPolicyBufferLifecycle, AccessPolicySecurityScope, DirectIoAlignmentRequirement,
    MixedAccessCoherenceBasis, MixedAccessTransition, MmapFaultPosture, PageCachePolicyProof,
    StoreAccessMode, StoreAccessOperation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyRequest {
    mode: StoreAccessMode,
    operation: StoreAccessOperation,
    reference: Option<PhysicalReference>,
    security_scope: Option<AccessPolicySecurityScope>,
    alignment: Option<DirectIoAlignmentRequirement>,
    buffer_lifecycle: Option<AccessPolicyBufferLifecycle>,
    page_cache_policy: Option<PageCachePolicyProof>,
    mixed_transition: Option<MixedAccessTransition>,
    coherence_basis: Option<MixedAccessCoherenceBasis>,
    mmap_fault_posture: MmapFaultPosture,
    required_evidence: CapabilityEvidenceClass,
}

impl AccessPolicyRequest {
    pub const fn buffered_read() -> Self {
        Self::new(StoreAccessMode::Buffered, StoreAccessOperation::Read)
    }

    pub const fn mmap_read() -> Self {
        Self::new(StoreAccessMode::Mmap, StoreAccessOperation::Read)
    }

    pub const fn direct_io_read() -> Self {
        Self::new(StoreAccessMode::DirectIo, StoreAccessOperation::Read)
    }

    pub const fn mixed_read(transition: MixedAccessTransition) -> Self {
        Self::new(StoreAccessMode::Mixed, StoreAccessOperation::Read)
            .with_mixed_transition(transition)
    }

    pub const fn new(mode: StoreAccessMode, operation: StoreAccessOperation) -> Self {
        Self {
            mode,
            operation,
            reference: None,
            security_scope: None,
            alignment: None,
            buffer_lifecycle: None,
            page_cache_policy: None,
            mixed_transition: None,
            coherence_basis: None,
            mmap_fault_posture: MmapFaultPosture::not_mmap(),
            required_evidence: CapabilityEvidenceClass::ExternallyGuaranteed,
        }
    }

    pub const fn for_physical_reference(mut self, reference: PhysicalReference) -> Self {
        self.reference = Some(reference);
        self
    }

    pub const fn with_security_scope(mut self, scope: AccessPolicySecurityScope) -> Self {
        self.security_scope = Some(scope);
        self
    }

    pub const fn with_alignment_requirement(
        mut self,
        alignment: DirectIoAlignmentRequirement,
    ) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub const fn with_buffer_lifecycle(mut self, lifecycle: AccessPolicyBufferLifecycle) -> Self {
        self.buffer_lifecycle = Some(lifecycle);
        self
    }

    pub const fn with_page_cache_policy(mut self, proof: PageCachePolicyProof) -> Self {
        self.page_cache_policy = Some(proof);
        self
    }

    pub const fn with_mmap_fault_posture(mut self, posture: MmapFaultPosture) -> Self {
        self.mmap_fault_posture = posture;
        self
    }

    pub const fn with_coherence_basis(mut self, basis: MixedAccessCoherenceBasis) -> Self {
        self.coherence_basis = Some(basis);
        self
    }

    pub const fn requiring_evidence(mut self, evidence: CapabilityEvidenceClass) -> Self {
        self.required_evidence = evidence;
        self
    }

    const fn with_mixed_transition(mut self, transition: MixedAccessTransition) -> Self {
        self.mixed_transition = Some(transition);
        self
    }

    pub const fn mode(self) -> StoreAccessMode {
        self.mode
    }
    pub const fn operation(self) -> StoreAccessOperation {
        self.operation
    }
    pub const fn reference(self) -> Option<PhysicalReference> {
        self.reference
    }
    pub const fn security_scope(self) -> Option<AccessPolicySecurityScope> {
        self.security_scope
    }
    pub const fn alignment(self) -> Option<DirectIoAlignmentRequirement> {
        self.alignment
    }
    pub const fn buffer_lifecycle(self) -> Option<AccessPolicyBufferLifecycle> {
        self.buffer_lifecycle
    }
    pub const fn page_cache_policy(self) -> Option<PageCachePolicyProof> {
        self.page_cache_policy
    }
    pub const fn mixed_transition(self) -> Option<MixedAccessTransition> {
        self.mixed_transition
    }
    pub const fn coherence_basis(self) -> Option<MixedAccessCoherenceBasis> {
        self.coherence_basis
    }
    pub const fn mmap_fault_posture(self) -> MmapFaultPosture {
        self.mmap_fault_posture
    }
    pub const fn required_evidence(self) -> CapabilityEvidenceClass {
        self.required_evidence
    }
}
