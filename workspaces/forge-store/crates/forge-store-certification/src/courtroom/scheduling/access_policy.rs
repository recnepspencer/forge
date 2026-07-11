use forge_store_physical_backend::{
    AccessPolicyCounterSnapshot, AccessPolicyDenial, AccessPolicyDenialKind,
    AccessPolicyExecutionReceipt, AccessPolicyViolation, AccessPolicyViolationKind,
    AdmittedAccessPolicy, BackendTargetProfile, CapabilityEvidenceClass, StoreAccessMode,
};
use forge_store_security::StoreSecurityScopeIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6AccessPolicyEvidenceOutcomeKind {
    Admitted,
    Executed,
    Denied(AccessPolicyDenialKind),
    Violated(AccessPolicyViolationKind),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6AccessPolicyEvidenceRow {
    mode: StoreAccessMode,
    profile: Option<BackendTargetProfile>,
    evidence_class: Option<CapabilityEvidenceClass>,
    security_scope: Option<StoreSecurityScopeIdentity>,
    outcome: S6AccessPolicyEvidenceOutcomeKind,
    counters: AccessPolicyCounterSnapshot,
}

impl S6AccessPolicyEvidenceRow {
    pub fn from_admitted(policy: AdmittedAccessPolicy) -> Self {
        Self {
            mode: policy.mode(),
            profile: Some(policy.profile()),
            evidence_class: Some(policy.evidence_class()),
            security_scope: policy.security_scope().map(|scope| scope.identity()),
            outcome: S6AccessPolicyEvidenceOutcomeKind::Admitted,
            counters: policy.counters(),
        }
    }

    pub fn from_execution_receipt(receipt: AccessPolicyExecutionReceipt) -> Self {
        let policy = receipt.policy();
        Self {
            mode: policy.mode(),
            profile: Some(policy.profile()),
            evidence_class: Some(policy.evidence_class()),
            security_scope: policy.security_scope().map(|scope| scope.identity()),
            outcome: S6AccessPolicyEvidenceOutcomeKind::Executed,
            counters: receipt.counters(),
        }
    }

    pub fn from_denial(mode: StoreAccessMode, denial: AccessPolicyDenial) -> Self {
        Self {
            mode,
            profile: None,
            evidence_class: None,
            security_scope: None,
            outcome: S6AccessPolicyEvidenceOutcomeKind::Denied(denial.kind()),
            counters: denial.counters(),
        }
    }

    pub fn from_violation(mode: StoreAccessMode, violation: AccessPolicyViolation) -> Self {
        Self {
            mode,
            profile: None,
            evidence_class: None,
            security_scope: None,
            outcome: S6AccessPolicyEvidenceOutcomeKind::Violated(violation.kind()),
            counters: violation.counters(),
        }
    }

    pub const fn mode(self) -> StoreAccessMode {
        self.mode
    }
    pub const fn profile(self) -> Option<BackendTargetProfile> {
        self.profile
    }
    pub const fn evidence_class(self) -> Option<CapabilityEvidenceClass> {
        self.evidence_class
    }
    pub const fn security_scope(self) -> Option<StoreSecurityScopeIdentity> {
        self.security_scope
    }
    pub const fn outcome(self) -> S6AccessPolicyEvidenceOutcomeKind {
        self.outcome
    }
    pub const fn counters(self) -> AccessPolicyCounterSnapshot {
        self.counters
    }
}
