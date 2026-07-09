use crate::{
    BufferPoolAuthorityRecap, PhysicalAuthorityRecap, S2BoundedCounterRecap, S2DenialBehaviorRecap,
    S3ReadinessDenial, S3ReadinessDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedIntegrityViewCapability {
    protected_view_count: u32,
}

impl ProtectedIntegrityViewCapability {
    pub fn protected_views(protected_view_count: u32) -> Result<Self, S3ReadinessDenial> {
        if protected_view_count == 0 {
            Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::MissingProtectedViewCapability,
            ))
        } else {
            Ok(Self {
                protected_view_count,
            })
        }
    }

    pub const fn protected_view_count(self) -> u32 {
        self.protected_view_count
    }

    pub const fn is_concrete(self) -> bool {
        self.protected_view_count > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifierResidentEnvelope {
    resident_bytes: u64,
    pinned_pages: u32,
}

impl VerifierResidentEnvelope {
    pub fn bounded(resident_bytes: u64, pinned_pages: u32) -> Result<Self, S3ReadinessDenial> {
        if resident_bytes == 0 || pinned_pages == 0 {
            Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::MissingVerifierResidentEnvelope,
            ))
        } else {
            Ok(Self {
                resident_bytes,
                pinned_pages,
            })
        }
    }

    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub const fn pinned_pages(self) -> u32 {
        self.pinned_pages
    }

    pub const fn is_bounded(self) -> bool {
        self.resident_bytes > 0 && self.pinned_pages > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubPlanningAllocationEnvelope {
    allocation_bytes: u64,
}

impl ScrubPlanningAllocationEnvelope {
    pub fn bounded(allocation_bytes: u64) -> Result<Self, S3ReadinessDenial> {
        if allocation_bytes == 0 {
            Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::MissingScrubAllocationEnvelope,
            ))
        } else {
            Ok(Self { allocation_bytes })
        }
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn is_bounded(self) -> bool {
        self.allocation_bytes > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityInspectionLifetimeLaw {
    lease_scoped: bool,
    indefinite_pin_allowed: bool,
}

impl IntegrityInspectionLifetimeLaw {
    pub fn lease_scoped() -> Self {
        Self {
            lease_scoped: true,
            indefinite_pin_allowed: false,
        }
    }

    pub const fn is_lease_scoped(self) -> bool {
        self.lease_scoped && !self.indefinite_pin_allowed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2NoMaterializationWitness {
    whole_store_materialization_attempts: u64,
    whole_object_materialization_attempts: u64,
}

impl S2NoMaterializationWitness {
    pub fn observed_zero(
        whole_store_materialization_attempts: u64,
        whole_object_materialization_attempts: u64,
    ) -> Result<Self, S3ReadinessDenial> {
        if whole_store_materialization_attempts == 0 && whole_object_materialization_attempts == 0 {
            Ok(Self {
                whole_store_materialization_attempts,
                whole_object_materialization_attempts,
            })
        } else {
            Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::MissingNoMaterializationWitness,
            ))
        }
    }

    pub const fn forbids_whole_store(self) -> bool {
        self.whole_store_materialization_attempts == 0
    }

    pub const fn forbids_whole_object(self) -> bool {
        self.whole_object_materialization_attempts == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S3PhysicalIntegrityReadinessPayload {
    protected_view_capability: ProtectedIntegrityViewCapability,
    verifier_resident_envelope: VerifierResidentEnvelope,
    scrub_allocation_envelope: ScrubPlanningAllocationEnvelope,
    inspection_lifetime_law: IntegrityInspectionLifetimeLaw,
    no_materialization_witness: S2NoMaterializationWitness,
    counter_recap: S2BoundedCounterRecap,
    denial_behavior: S2DenialBehaviorRecap,
    physical_authority_recap: PhysicalAuthorityRecap,
    buffer_pool_authority_recap: BufferPoolAuthorityRecap,
    later_sequence_semantic_claimed: bool,
}

impl S3PhysicalIntegrityReadinessPayload {
    #[allow(clippy::too_many_arguments)]
    pub fn from_s2_closeout_evidence(
        protected_view_capability: ProtectedIntegrityViewCapability,
        verifier_resident_envelope: VerifierResidentEnvelope,
        scrub_allocation_envelope: ScrubPlanningAllocationEnvelope,
        inspection_lifetime_law: IntegrityInspectionLifetimeLaw,
        no_materialization_witness: S2NoMaterializationWitness,
        counter_recap: S2BoundedCounterRecap,
        denial_behavior: S2DenialBehaviorRecap,
        physical_authority_recap: PhysicalAuthorityRecap,
        buffer_pool_authority_recap: BufferPoolAuthorityRecap,
    ) -> Self {
        Self {
            protected_view_capability,
            verifier_resident_envelope,
            scrub_allocation_envelope,
            inspection_lifetime_law,
            no_materialization_witness,
            counter_recap,
            denial_behavior,
            physical_authority_recap,
            buffer_pool_authority_recap,
            later_sequence_semantic_claimed: false,
        }
    }

    pub fn require_complete(self) -> Result<Self, S3ReadinessDenial> {
        require(
            self.protected_view_capability.is_concrete(),
            S3ReadinessDenialKind::MissingProtectedViewCapability,
        )?;
        require(
            self.verifier_resident_envelope.is_bounded(),
            S3ReadinessDenialKind::MissingVerifierResidentEnvelope,
        )?;
        require(
            self.scrub_allocation_envelope.is_bounded(),
            S3ReadinessDenialKind::MissingScrubAllocationEnvelope,
        )?;
        require(
            self.inspection_lifetime_law.is_lease_scoped(),
            S3ReadinessDenialKind::MissingInspectionLifetimeLaw,
        )?;
        require(
            self.no_materialization_witness.forbids_whole_store()
                && self.no_materialization_witness.forbids_whole_object(),
            S3ReadinessDenialKind::MissingNoMaterializationWitness,
        )?;
        require(
            self.denial_behavior.named_denial_count() == 6,
            S3ReadinessDenialKind::MissingDenialBehavior,
        )?;
        require(
            !self.later_sequence_semantic_claimed,
            S3ReadinessDenialKind::LaterSequenceSemanticClaimed,
        )?;
        Ok(self)
    }

    pub const fn protected_view_capability(self) -> ProtectedIntegrityViewCapability {
        self.protected_view_capability
    }

    pub const fn verifier_resident_envelope(self) -> VerifierResidentEnvelope {
        self.verifier_resident_envelope
    }

    pub const fn scrub_allocation_envelope(self) -> ScrubPlanningAllocationEnvelope {
        self.scrub_allocation_envelope
    }

    pub const fn inspection_lifetime_law(self) -> IntegrityInspectionLifetimeLaw {
        self.inspection_lifetime_law
    }

    pub const fn no_materialization_witness(self) -> S2NoMaterializationWitness {
        self.no_materialization_witness
    }

    pub const fn counter_recap(self) -> S2BoundedCounterRecap {
        self.counter_recap
    }

    pub const fn denial_behavior(self) -> S2DenialBehaviorRecap {
        self.denial_behavior
    }

    pub const fn physical_authority_recap(self) -> PhysicalAuthorityRecap {
        self.physical_authority_recap
    }

    pub const fn buffer_pool_authority_recap(self) -> BufferPoolAuthorityRecap {
        self.buffer_pool_authority_recap
    }

    pub const fn claims_later_sequence_semantics(self) -> bool {
        self.later_sequence_semantic_claimed
    }
}

fn require(condition: bool, denial: S3ReadinessDenialKind) -> Result<(), S3ReadinessDenial> {
    if condition {
        Ok(())
    } else {
        Err(S3ReadinessDenial::new(denial))
    }
}
