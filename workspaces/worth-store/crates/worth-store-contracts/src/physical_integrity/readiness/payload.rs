use crate::{
    BoundedCounterRecap, BufferPoolAuthorityRecap, DenialBehaviorRecap, PhysicalAuthorityRecap,
    PhysicalIntegrityReadinessDenial, PhysicalIntegrityReadinessDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedIntegrityViewCapability {
    protected_view_count: u32,
}

impl ProtectedIntegrityViewCapability {
    pub fn protected_views(
        protected_view_count: u32,
    ) -> Result<Self, PhysicalIntegrityReadinessDenial> {
        if protected_view_count == 0 {
            Err(PhysicalIntegrityReadinessDenial::new(
                PhysicalIntegrityReadinessDenialKind::MissingProtectedViewCapability,
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
    pub fn bounded(
        resident_bytes: u64,
        pinned_pages: u32,
    ) -> Result<Self, PhysicalIntegrityReadinessDenial> {
        if resident_bytes == 0 || pinned_pages == 0 {
            Err(PhysicalIntegrityReadinessDenial::new(
                PhysicalIntegrityReadinessDenialKind::MissingVerifierResidentEnvelope,
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
    pub fn bounded(allocation_bytes: u64) -> Result<Self, PhysicalIntegrityReadinessDenial> {
        if allocation_bytes == 0 {
            Err(PhysicalIntegrityReadinessDenial::new(
                PhysicalIntegrityReadinessDenialKind::MissingScrubAllocationEnvelope,
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
pub struct NoMaterializationWitness {
    whole_store_materialization_attempts: u64,
    whole_object_materialization_attempts: u64,
}

impl NoMaterializationWitness {
    pub fn observed_zero(
        whole_store_materialization_attempts: u64,
        whole_object_materialization_attempts: u64,
    ) -> Result<Self, PhysicalIntegrityReadinessDenial> {
        if whole_store_materialization_attempts == 0 && whole_object_materialization_attempts == 0 {
            Ok(Self {
                whole_store_materialization_attempts,
                whole_object_materialization_attempts,
            })
        } else {
            Err(PhysicalIntegrityReadinessDenial::new(
                PhysicalIntegrityReadinessDenialKind::MissingNoMaterializationWitness,
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
pub struct PhysicalIntegrityReadinessPayload {
    protected_view_capability: ProtectedIntegrityViewCapability,
    verifier_resident_envelope: VerifierResidentEnvelope,
    scrub_allocation_envelope: ScrubPlanningAllocationEnvelope,
    inspection_lifetime_law: IntegrityInspectionLifetimeLaw,
    no_materialization_witness: NoMaterializationWitness,
    counter_recap: BoundedCounterRecap,
    denial_behavior: DenialBehaviorRecap,
    physical_authority_recap: PhysicalAuthorityRecap,
    buffer_pool_authority_recap: BufferPoolAuthorityRecap,
    later_sequence_semantic_claimed: bool,
}

impl PhysicalIntegrityReadinessPayload {
    #[allow(clippy::too_many_arguments)]
    pub fn from_physical_substrate_closeout_evidence(
        protected_view_capability: ProtectedIntegrityViewCapability,
        verifier_resident_envelope: VerifierResidentEnvelope,
        scrub_allocation_envelope: ScrubPlanningAllocationEnvelope,
        inspection_lifetime_law: IntegrityInspectionLifetimeLaw,
        no_materialization_witness: NoMaterializationWitness,
        counter_recap: BoundedCounterRecap,
        denial_behavior: DenialBehaviorRecap,
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

    pub fn require_complete(self) -> Result<Self, PhysicalIntegrityReadinessDenial> {
        require(
            self.protected_view_capability.is_concrete(),
            PhysicalIntegrityReadinessDenialKind::MissingProtectedViewCapability,
        )?;
        require(
            self.verifier_resident_envelope.is_bounded(),
            PhysicalIntegrityReadinessDenialKind::MissingVerifierResidentEnvelope,
        )?;
        require(
            self.scrub_allocation_envelope.is_bounded(),
            PhysicalIntegrityReadinessDenialKind::MissingScrubAllocationEnvelope,
        )?;
        require(
            self.inspection_lifetime_law.is_lease_scoped(),
            PhysicalIntegrityReadinessDenialKind::MissingInspectionLifetimeLaw,
        )?;
        require(
            self.no_materialization_witness.forbids_whole_store()
                && self.no_materialization_witness.forbids_whole_object(),
            PhysicalIntegrityReadinessDenialKind::MissingNoMaterializationWitness,
        )?;
        require(
            self.denial_behavior.named_denial_count() == 6,
            PhysicalIntegrityReadinessDenialKind::MissingDenialBehavior,
        )?;
        require(
            !self.later_sequence_semantic_claimed,
            PhysicalIntegrityReadinessDenialKind::LaterSequenceSemanticClaimed,
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

    pub const fn no_materialization_witness(self) -> NoMaterializationWitness {
        self.no_materialization_witness
    }

    pub const fn counter_recap(self) -> BoundedCounterRecap {
        self.counter_recap
    }

    pub const fn denial_behavior(self) -> DenialBehaviorRecap {
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

fn require(
    condition: bool,
    denial: PhysicalIntegrityReadinessDenialKind,
) -> Result<(), PhysicalIntegrityReadinessDenial> {
    if condition {
        Ok(())
    } else {
        Err(PhysicalIntegrityReadinessDenial::new(denial))
    }
}
