use worth_store_physical_format::{PhysicalReclaimRegion, ReclaimedByteInterpretation};

use super::{ReclaimPolicyCounterSnapshot, ReclaimPolicySecurityScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimPolicyExecutionObservation {
    observed_region: PhysicalReclaimRegion,
    observed_interpretation: ReclaimedByteInterpretation,
    observed_security_scope: ReclaimPolicySecurityScope,
    non_claim_handoff_preserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReclaimPolicyViolation {
    kind: ReclaimPolicyViolationKind,
    counters: ReclaimPolicyCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReclaimPolicyViolationKind {
    ProtectedReachabilityLost,
    SecurityScopeLost,
    ByteInterpretationContradicted {
        admitted: ReclaimedByteInterpretation,
        observed: ReclaimedByteInterpretation,
    },
    LaterHandoffStrengthened,
}

impl ReclaimPolicyExecutionObservation {
    pub const fn new(
        observed_region: PhysicalReclaimRegion,
        observed_interpretation: ReclaimedByteInterpretation,
        observed_security_scope: ReclaimPolicySecurityScope,
        non_claim_handoff_preserved: bool,
    ) -> Self {
        Self {
            observed_region,
            observed_interpretation,
            observed_security_scope,
            non_claim_handoff_preserved,
        }
    }

    pub const fn observed_region(&self) -> PhysicalReclaimRegion {
        self.observed_region
    }

    pub const fn observed_interpretation(&self) -> ReclaimedByteInterpretation {
        self.observed_interpretation
    }

    pub const fn observed_security_scope(&self) -> ReclaimPolicySecurityScope {
        self.observed_security_scope
    }

    pub const fn non_claim_handoff_preserved(&self) -> bool {
        self.non_claim_handoff_preserved
    }
}

impl ReclaimPolicyViolation {
    pub const fn new(
        kind: ReclaimPolicyViolationKind,
        counters: ReclaimPolicyCounterSnapshot,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> ReclaimPolicyViolationKind {
        self.kind
    }

    pub const fn counters(self) -> ReclaimPolicyCounterSnapshot {
        self.counters
    }
}
