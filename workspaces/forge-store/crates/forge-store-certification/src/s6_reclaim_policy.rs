use forge_store_blob_chunks::S6BlobReclaimNonClaimHandoff;
use forge_store_physical_format::ReclaimedByteInterpretation;
use forge_store_reclaim_policy::{
    AdmittedReclaimPolicy, ReclaimPolicyCounterSnapshot, ReclaimPolicyDenial,
    ReclaimPolicyDenialKind, ReclaimPolicyExecutionReceipt, ReclaimPolicyOperation,
    ReclaimPolicyViolation, ReclaimPolicyViolationKind,
};
use forge_store_security::StoreSecurityScopeIdentity;
use forge_store_tiering::S6ColdTierIoPosture;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum S6ReclaimPolicyEvidenceOutcomeKind {
    Admitted,
    Executed,
    Denied(ReclaimPolicyDenialKind),
    Violated(ReclaimPolicyViolationKind),
    BlobNonClaimHandoff,
    ColdTierNonClaimHandoff,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S6ReclaimPolicyEvidenceRow {
    operation: Option<ReclaimPolicyOperation>,
    interpretation: Option<ReclaimedByteInterpretation>,
    security_scope: Option<StoreSecurityScopeIdentity>,
    outcome: S6ReclaimPolicyEvidenceOutcomeKind,
    counters: ReclaimPolicyCounterSnapshot,
}

impl S6ReclaimPolicyEvidenceRow {
    pub fn from_admitted(policy: AdmittedReclaimPolicy) -> Self {
        Self {
            operation: Some(policy.posture().operation()),
            interpretation: Some(policy.posture().interpretation()),
            security_scope: Some(policy.security_scope().identity()),
            outcome: S6ReclaimPolicyEvidenceOutcomeKind::Admitted,
            counters: policy.counters(),
        }
    }

    pub fn from_execution_receipt(receipt: ReclaimPolicyExecutionReceipt) -> Self {
        let policy = receipt.policy();
        Self {
            operation: Some(policy.posture().operation()),
            interpretation: Some(receipt.observed_interpretation()),
            security_scope: Some(policy.security_scope().identity()),
            outcome: S6ReclaimPolicyEvidenceOutcomeKind::Executed,
            counters: receipt.counters(),
        }
    }

    pub fn from_denial(denial: ReclaimPolicyDenial) -> Self {
        Self {
            operation: None,
            interpretation: None,
            security_scope: None,
            outcome: S6ReclaimPolicyEvidenceOutcomeKind::Denied(denial.kind().clone()),
            counters: denial.counters(),
        }
    }

    pub fn from_violation(violation: ReclaimPolicyViolation) -> Self {
        Self {
            operation: None,
            interpretation: None,
            security_scope: None,
            outcome: S6ReclaimPolicyEvidenceOutcomeKind::Violated(violation.kind()),
            counters: violation.counters(),
        }
    }

    pub fn from_blob_non_claim_handoff(handoff: S6BlobReclaimNonClaimHandoff) -> Self {
        Self {
            operation: None,
            interpretation: Some(handoff.interpretation()),
            security_scope: Some(handoff.security_scope()),
            outcome: S6ReclaimPolicyEvidenceOutcomeKind::BlobNonClaimHandoff,
            counters: handoff.counters(),
        }
    }

    pub fn from_cold_tier_non_claim_handoff(posture: S6ColdTierIoPosture) -> Self {
        Self {
            operation: Some(ReclaimPolicyOperation::ColdTierMovementPosture),
            interpretation: Some(posture.interpretation()),
            security_scope: Some(posture.security_scope()),
            outcome: S6ReclaimPolicyEvidenceOutcomeKind::ColdTierNonClaimHandoff,
            counters: posture.counters(),
        }
    }

    pub const fn operation(&self) -> Option<ReclaimPolicyOperation> {
        self.operation
    }

    pub const fn interpretation(&self) -> Option<ReclaimedByteInterpretation> {
        self.interpretation
    }

    pub const fn security_scope(&self) -> Option<StoreSecurityScopeIdentity> {
        self.security_scope
    }

    pub const fn outcome(&self) -> &S6ReclaimPolicyEvidenceOutcomeKind {
        &self.outcome
    }

    pub const fn counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.counters
    }
}
