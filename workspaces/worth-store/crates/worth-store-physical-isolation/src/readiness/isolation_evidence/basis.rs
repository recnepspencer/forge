use worth_foundational::performance_api::lower_lane::receipts::FoundationalPerformanceCounterRow;
use worth_foundational::{
    FoundationalAuthoritativePerformanceClaim, FoundationalBoundaryEvidenceResidualDebt,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalCounterBackedPerformanceReceipt,
};
use worth_proof::{
    Admitted, AssumptionBasis, AuthorityMarker, AuthorityWitness,
    BoundaryBridgedAuthorityRevalidationRequiredBasis, CapabilityMarker, CapabilityWitness,
    CurrentValidity, FreshnessScopedBasis, Lowered, Recipe, Resolved, Unresolved,
};

use crate::PhysicalIsolationCounterSnapshot;

pub type FoundationalIsolationCounterReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutedIsolationBasis {
    executed_isolation_identity: u64,
    counter_identity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerIsolationCapabilityBasis {
    closeout_basis: ExecutedIsolationBasis,
    foundational_counter_receipt_identity: u64,
    proof_progression_identity: u64,
    projection_evidence: IsolationEvidenceProjection,
    assumption_basis: AssumptionBasis<ExecutedIsolationBasis>,
}

pub type SchedulerIsolationFreshBasis =
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<SchedulerIsolationCapabilityBasis>>;
pub type SchedulerIsolationResolvedRecipe =
    Recipe<Resolved, SchedulerIsolationCapabilityProofRequest, SchedulerIsolationFreshBasis>;
pub type SchedulerIsolationLoweredRecipe =
    Recipe<Lowered, SchedulerIsolationCapabilityProofRequest, SchedulerIsolationFreshBasis>;
pub type SchedulerIsolationAdmittedRecipe =
    Recipe<Admitted, SchedulerIsolationCapabilityProofRequest, SchedulerIsolationFreshBasis>;
pub type SchedulerIsolationBoundaryBridgedRecipe = Recipe<
    Admitted,
    SchedulerIsolationCapabilityProofRequest,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<SchedulerIsolationCapabilityBasis>,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerIsolationCapabilityProofRequest {
    closeout_basis: ExecutedIsolationBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerIsolationProof {
    resolved: SchedulerIsolationResolvedRecipe,
    lowered: SchedulerIsolationLoweredRecipe,
    admitted: SchedulerIsolationAdmittedRecipe,
    bridged: SchedulerIsolationBoundaryBridgedRecipe,
    readmitted: SchedulerIsolationAdmittedRecipe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationEvidenceProjection {
    foundational_counter_receipt_identity: u64,
    foundational_counter_receipt: FoundationalIsolationCounterReceipt,
    proof_progression_identity: u64,
    support_basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    residual_debt: [FoundationalBoundaryEvidenceResidualDebt; 3],
    authority_posture: SchedulerIsolationAuthorityPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerIsolationAuthorityPosture {
    StoreExecutedIsolationMaterialized,
    FoundationalAndProofProjectionOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerIsolationPublicationAuthority {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerIsolationPublicationWitness {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SchedulerIsolationLoweringCapability {
    _private: (),
}

impl AuthorityMarker for SchedulerIsolationPublicationAuthority {}
impl CapabilityMarker for SchedulerIsolationLoweringCapability {}

impl ExecutedIsolationBasis {
    pub const fn from_executed_isolation(
        executed_isolation_identity: u64,
        counters: PhysicalIsolationCounterSnapshot,
    ) -> Self {
        Self {
            executed_isolation_identity,
            counter_identity: counter_identity(counters),
        }
    }

    pub const fn executed_isolation_identity(self) -> u64 {
        self.executed_isolation_identity
    }

    pub const fn counter_identity(self) -> u64 {
        self.counter_identity
    }
}

impl SchedulerIsolationCapabilityBasis {
    pub fn from_closeout_basis(
        closeout_basis: ExecutedIsolationBasis,
        projection_evidence: IsolationEvidenceProjection,
    ) -> Self {
        Self {
            closeout_basis,
            foundational_counter_receipt_identity: projection_evidence
                .foundational_counter_receipt_identity(),
            proof_progression_identity: projection_evidence.proof_progression_identity(),
            projection_evidence,
            assumption_basis: AssumptionBasis::new(closeout_basis),
        }
    }

    pub const fn closeout_basis(&self) -> ExecutedIsolationBasis {
        self.closeout_basis
    }

    pub const fn foundational_counter_receipt_identity(&self) -> u64 {
        self.foundational_counter_receipt_identity
    }

    pub const fn proof_progression_identity(&self) -> u64 {
        self.proof_progression_identity
    }

    pub const fn assumption_basis(&self) -> &AssumptionBasis<ExecutedIsolationBasis> {
        &self.assumption_basis
    }

    pub const fn projection_evidence(&self) -> &IsolationEvidenceProjection {
        &self.projection_evidence
    }
}

impl SchedulerIsolationCapabilityProofRequest {
    pub const fn closeout_basis(&self) -> ExecutedIsolationBasis {
        self.closeout_basis
    }
}

impl SchedulerIsolationProof {
    pub(crate) fn from_basis(
        basis: SchedulerIsolationCapabilityBasis,
        witness: SchedulerIsolationPublicationWitness,
    ) -> Self {
        let request = SchedulerIsolationCapabilityProofRequest {
            closeout_basis: basis.closeout_basis(),
        };
        let resolved = Recipe::<Unresolved, _>::new(request)
            .resolve_with_authority(basis.clone(), witness.authority_witness());
        let lowered = resolved
            .clone()
            .lower_with_capability(scheduler_isolation_lowering_capability());
        let admitted = lowered
            .clone()
            .admit_with_authority(witness.authority_witness());
        let bridged = admitted.clone().bridge_trust_boundary();
        let readmitted = bridged
            .clone()
            .readmit_with_authority(basis, witness.authority_witness());
        Self {
            resolved,
            lowered,
            admitted,
            bridged,
            readmitted,
        }
    }

    pub const fn resolved_recipe(&self) -> &SchedulerIsolationResolvedRecipe {
        &self.resolved
    }

    pub const fn lowered_recipe(&self) -> &SchedulerIsolationLoweredRecipe {
        &self.lowered
    }

    pub const fn admitted_recipe(&self) -> &SchedulerIsolationAdmittedRecipe {
        &self.admitted
    }

    pub const fn boundary_bridged_recipe(&self) -> &SchedulerIsolationBoundaryBridgedRecipe {
        &self.bridged
    }

    pub const fn readmitted_recipe(&self) -> &SchedulerIsolationAdmittedRecipe {
        &self.readmitted
    }
}

impl SchedulerIsolationPublicationWitness {
    pub(crate) fn from_validated_store_handoff(
        counters: PhysicalIsolationCounterSnapshot,
        projection_evidence: &IsolationEvidenceProjection,
    ) -> Result<Self, crate::IsolationReadinessDenial> {
        let _ = PhysicalIsolationCounterSnapshot::from_store_executed_counts(
            counters.outcome_count(),
            counters.wait_count(),
            counters.retry_count(),
            counters.latch_counter_rows(),
            counters.latch_wait_count(),
            counters.reclaim_counter_rows(),
            counters.blocked_maintenance_count(),
            counters.reclaim_block_count(),
            counters.protected_byte_footprint(),
        )?;
        if projection_evidence.authority_posture()
            != SchedulerIsolationAuthorityPosture::StoreExecutedIsolationMaterialized
        {
            return Err(crate::IsolationReadinessDenial::LogOrTerminalProjection);
        }
        Ok(Self { _private: () })
    }

    fn authority_witness(self) -> AuthorityWitness<SchedulerIsolationPublicationAuthority> {
        AuthorityWitness::from_authority_marker(SchedulerIsolationPublicationAuthority {
            _private: (),
        })
    }
}

impl IsolationEvidenceProjection {
    pub(crate) fn from_store_executed_projection(
        foundational_counter_receipt: FoundationalIsolationCounterReceipt,
        proof_progression_identity: u64,
    ) -> Self {
        let foundational_counter_receipt_identity =
            foundational_receipt_identity(foundational_counter_receipt.counter_rows());
        Self {
            foundational_counter_receipt_identity,
            foundational_counter_receipt,
            proof_progression_identity,
            support_basis_disclosure:
                FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis,
            residual_debt: [
                FoundationalBoundaryEvidenceResidualDebt::AdoptingRuntimeParityDeferred,
                FoundationalBoundaryEvidenceResidualDebt::RuntimeSpecificHistoryAndJournalTaxonomiesDeferred,
                FoundationalBoundaryEvidenceResidualDebt::RealRuntimeSupportBundlePersistenceDeferred,
            ],
            authority_posture: SchedulerIsolationAuthorityPosture::StoreExecutedIsolationMaterialized,
        }
    }

    pub const fn foundational_counter_receipt_identity(&self) -> u64 {
        self.foundational_counter_receipt_identity
    }

    pub const fn foundational_counter_receipt(&self) -> &FoundationalIsolationCounterReceipt {
        &self.foundational_counter_receipt
    }

    pub const fn proof_progression_identity(&self) -> u64 {
        self.proof_progression_identity
    }

    pub const fn support_basis_disclosure(
        &self,
    ) -> FoundationalBoundaryEvidenceSupportBasisDisclosure {
        self.support_basis_disclosure
    }

    pub const fn residual_debt(&self) -> &[FoundationalBoundaryEvidenceResidualDebt; 3] {
        &self.residual_debt
    }

    pub const fn authority_posture(&self) -> SchedulerIsolationAuthorityPosture {
        self.authority_posture
    }
}

fn scheduler_isolation_lowering_capability(
) -> CapabilityWitness<SchedulerIsolationLoweringCapability> {
    CapabilityWitness::from_capability_marker(SchedulerIsolationLoweringCapability { _private: () })
}

const fn counter_identity(counters: PhysicalIsolationCounterSnapshot) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325_u64;
    digest = mix_u64(digest, counters.outcome_count());
    digest = mix_u64(digest, counters.wait_count());
    digest = mix_u64(digest, counters.retry_count());
    digest = mix_u64(digest, counters.latch_counter_rows());
    digest = mix_u64(digest, counters.latch_wait_count());
    digest = mix_u64(digest, counters.reclaim_counter_rows());
    digest = mix_u64(digest, counters.blocked_maintenance_count());
    digest = mix_u64(digest, counters.reclaim_block_count());
    mix_u64(digest, counters.protected_byte_footprint())
}

fn foundational_receipt_identity(rows: &[FoundationalPerformanceCounterRow]) -> u64 {
    rows.iter().fold(0x6f58_0000_0000_0001, |digest, row| {
        let digest = mix_text(digest, row.name().as_str());
        mix_u64(digest, row.observed_count())
    })
}

fn mix_text(mut digest: u64, text: &str) -> u64 {
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x1000_0000_01b3);
    }
    digest
}

const fn mix_u64(mut digest: u64, value: u64) -> u64 {
    let bytes = value.to_le_bytes();
    let mut index = 0;
    while index < bytes.len() {
        digest ^= bytes[index] as u64;
        digest = digest.wrapping_mul(0x1000_0000_01b3);
        index += 1;
    }
    digest
}
