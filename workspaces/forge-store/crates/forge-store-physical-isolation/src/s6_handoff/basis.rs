use forge_foundational::performance_api::lower_lane::receipts::FoundationalPerformanceCounterRow;
use forge_foundational::{
    FoundationalAuthoritativePerformanceClaim, FoundationalBoundaryEvidenceResidualDebt,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalCounterBackedPerformanceReceipt,
};
use forge_proof::{
    Admitted, AssumptionBasis, AuthorityMarker, AuthorityWitness,
    BoundaryBridgedAuthorityRevalidationRequiredBasis, CapabilityMarker, CapabilityWitness,
    CurrentValidity, FreshnessScopedBasis, Lowered, Recipe, Resolved, Unresolved,
};

use super::PhysicalIsolationCounterSnapshot;

pub type S6FoundationalCounterReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5PhysicalIsolationCloseoutBasis {
    executed_isolation_identity: u64,
    counter_identity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6IoQosIsolationReadinessBasis {
    closeout_basis: S5PhysicalIsolationCloseoutBasis,
    foundational_counter_receipt_identity: u64,
    proof_progression_identity: u64,
    projection_evidence: S6HandoffProjectionEvidence,
    assumption_basis: AssumptionBasis<S5PhysicalIsolationCloseoutBasis>,
}

pub type S6ReadinessFreshBasis =
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<S6IoQosIsolationReadinessBasis>>;
pub type S6ReadinessResolvedRecipe =
    Recipe<Resolved, S6IoQosIsolationReadinessProofRequest, S6ReadinessFreshBasis>;
pub type S6ReadinessLoweredRecipe =
    Recipe<Lowered, S6IoQosIsolationReadinessProofRequest, S6ReadinessFreshBasis>;
pub type S6ReadinessAdmittedRecipe =
    Recipe<Admitted, S6IoQosIsolationReadinessProofRequest, S6ReadinessFreshBasis>;
pub type S6ReadinessBoundaryBridgedRecipe = Recipe<
    Admitted,
    S6IoQosIsolationReadinessProofRequest,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<S6IoQosIsolationReadinessBasis>,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6IoQosIsolationReadinessProofRequest {
    closeout_basis: S5PhysicalIsolationCloseoutBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6ReadinessProofHandoff {
    resolved: S6ReadinessResolvedRecipe,
    lowered: S6ReadinessLoweredRecipe,
    admitted: S6ReadinessAdmittedRecipe,
    bridged: S6ReadinessBoundaryBridgedRecipe,
    readmitted: S6ReadinessAdmittedRecipe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6HandoffProjectionEvidence {
    foundational_counter_receipt_identity: u64,
    foundational_counter_receipt: S6FoundationalCounterReceipt,
    proof_progression_identity: u64,
    support_basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    residual_debt: [FoundationalBoundaryEvidenceResidualDebt; 3],
    authority_posture: S6ReadinessAuthorityPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6ReadinessAuthorityPosture {
    StoreExecutedIsolationMaterialized,
    FoundationalAndProofProjectionOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6ReadinessPublicationAuthority {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6ReadinessPublicationWitness {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct S6ReadinessLoweringCapability {
    _private: (),
}

impl AuthorityMarker for S6ReadinessPublicationAuthority {}
impl CapabilityMarker for S6ReadinessLoweringCapability {}

impl S5PhysicalIsolationCloseoutBasis {
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

impl S6IoQosIsolationReadinessBasis {
    pub fn from_closeout_basis(
        closeout_basis: S5PhysicalIsolationCloseoutBasis,
        projection_evidence: S6HandoffProjectionEvidence,
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

    pub const fn closeout_basis(&self) -> S5PhysicalIsolationCloseoutBasis {
        self.closeout_basis
    }

    pub const fn foundational_counter_receipt_identity(&self) -> u64 {
        self.foundational_counter_receipt_identity
    }

    pub const fn proof_progression_identity(&self) -> u64 {
        self.proof_progression_identity
    }

    pub const fn assumption_basis(&self) -> &AssumptionBasis<S5PhysicalIsolationCloseoutBasis> {
        &self.assumption_basis
    }

    pub const fn projection_evidence(&self) -> &S6HandoffProjectionEvidence {
        &self.projection_evidence
    }
}

impl S6IoQosIsolationReadinessProofRequest {
    pub const fn closeout_basis(&self) -> S5PhysicalIsolationCloseoutBasis {
        self.closeout_basis
    }
}

impl S6ReadinessProofHandoff {
    pub(crate) fn from_basis(
        basis: S6IoQosIsolationReadinessBasis,
        witness: S6ReadinessPublicationWitness,
    ) -> Self {
        let request = S6IoQosIsolationReadinessProofRequest {
            closeout_basis: basis.closeout_basis(),
        };
        let resolved = Recipe::<Unresolved, _>::new(request)
            .resolve_with_authority(basis.clone(), witness.authority_witness());
        let lowered = resolved
            .clone()
            .lower_with_capability(s6_readiness_lowering_capability());
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

    pub const fn resolved_recipe(&self) -> &S6ReadinessResolvedRecipe {
        &self.resolved
    }

    pub const fn lowered_recipe(&self) -> &S6ReadinessLoweredRecipe {
        &self.lowered
    }

    pub const fn admitted_recipe(&self) -> &S6ReadinessAdmittedRecipe {
        &self.admitted
    }

    pub const fn boundary_bridged_recipe(&self) -> &S6ReadinessBoundaryBridgedRecipe {
        &self.bridged
    }

    pub const fn readmitted_recipe(&self) -> &S6ReadinessAdmittedRecipe {
        &self.readmitted
    }
}

impl S6ReadinessPublicationWitness {
    pub(crate) fn from_validated_store_handoff(
        counters: PhysicalIsolationCounterSnapshot,
        projection_evidence: &S6HandoffProjectionEvidence,
    ) -> Result<Self, super::S6IoQosIsolationReadinessDenial> {
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
            != S6ReadinessAuthorityPosture::StoreExecutedIsolationMaterialized
        {
            return Err(super::S6IoQosIsolationReadinessDenial::LogOrTerminalProjection);
        }
        Ok(Self { _private: () })
    }

    fn authority_witness(self) -> AuthorityWitness<S6ReadinessPublicationAuthority> {
        AuthorityWitness::from_authority_marker(S6ReadinessPublicationAuthority { _private: () })
    }
}

impl S6HandoffProjectionEvidence {
    pub(crate) fn from_store_executed_projection(
        foundational_counter_receipt: S6FoundationalCounterReceipt,
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
            authority_posture: S6ReadinessAuthorityPosture::StoreExecutedIsolationMaterialized,
        }
    }

    pub const fn foundational_counter_receipt_identity(&self) -> u64 {
        self.foundational_counter_receipt_identity
    }

    pub const fn foundational_counter_receipt(&self) -> &S6FoundationalCounterReceipt {
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

    pub const fn authority_posture(&self) -> S6ReadinessAuthorityPosture {
        self.authority_posture
    }
}

fn s6_readiness_lowering_capability() -> CapabilityWitness<S6ReadinessLoweringCapability> {
    CapabilityWitness::from_capability_marker(S6ReadinessLoweringCapability { _private: () })
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
