use crate::identity::hash_parts;

use super::counters::BasisEligibilityCounters;
use super::proofs::BasisEligibilityDecisionTrace;
use super::scoping::ScopedBasisProof;
use super::taxonomy::{BasisAuthorityPosture, DeniedBasisCapabilityKind};
use super::DeniedBasisCapability;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LowerRuntimeEvidenceAuthority {
    Runtime,
    RuntimeBridgeFacade,
    RelationalFacade,
    SignalFacade,
    Unsupported,
}

impl LowerRuntimeEvidenceAuthority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::RuntimeBridgeFacade => "runtime_bridge_facade",
            Self::RelationalFacade => "relational_facade",
            Self::SignalFacade => "signal_facade",
            Self::Unsupported => "unsupported",
        }
    }

    fn basis_authority_posture(&self) -> Option<BasisAuthorityPosture> {
        match self {
            Self::Runtime => Some(BasisAuthorityPosture::Runtime),
            Self::RuntimeBridgeFacade => Some(BasisAuthorityPosture::RuntimeBridgeFacade),
            Self::RelationalFacade => Some(BasisAuthorityPosture::RelationalFacade),
            Self::SignalFacade => Some(BasisAuthorityPosture::SignalFacade),
            Self::Unsupported => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBasisEvidence {
    authority: LowerRuntimeEvidenceAuthority,
    basis_digest: String,
    evidence_digest: String,
    retained_evidence_lookup_width: usize,
    stale_runtime_snapshot: bool,
    missing_signal_observation: bool,
    unsupported_capability: bool,
}

impl LowerRuntimeBasisEvidence {
    pub fn from_runtime_basis(
        basis_digest: impl Into<String>,
        evidence_digest: impl Into<String>,
        retained_evidence_lookup_width: usize,
    ) -> Self {
        Self::new(
            LowerRuntimeEvidenceAuthority::Runtime,
            basis_digest,
            evidence_digest,
            retained_evidence_lookup_width,
        )
    }

    pub fn from_runtime_bridge_facade(
        basis_digest: impl Into<String>,
        evidence_digest: impl Into<String>,
        retained_evidence_lookup_width: usize,
    ) -> Self {
        Self::new(
            LowerRuntimeEvidenceAuthority::RuntimeBridgeFacade,
            basis_digest,
            evidence_digest,
            retained_evidence_lookup_width,
        )
    }

    pub fn from_relational_facade(
        basis_digest: impl Into<String>,
        evidence_digest: impl Into<String>,
        retained_evidence_lookup_width: usize,
    ) -> Self {
        Self::new(
            LowerRuntimeEvidenceAuthority::RelationalFacade,
            basis_digest,
            evidence_digest,
            retained_evidence_lookup_width,
        )
    }

    pub fn from_signal_facade(
        basis_digest: impl Into<String>,
        evidence_digest: impl Into<String>,
        retained_evidence_lookup_width: usize,
    ) -> Self {
        Self::new(
            LowerRuntimeEvidenceAuthority::SignalFacade,
            basis_digest,
            evidence_digest,
            retained_evidence_lookup_width,
        )
    }

    pub fn stale_runtime_snapshot(
        basis_digest: impl Into<String>,
        evidence_digest: impl Into<String>,
        retained_evidence_lookup_width: usize,
    ) -> Self {
        let mut evidence = Self::from_runtime_bridge_facade(
            basis_digest,
            evidence_digest,
            retained_evidence_lookup_width,
        );
        evidence.stale_runtime_snapshot = true;
        evidence
    }

    pub fn missing_signal_observation(evidence_digest: impl Into<String>) -> Self {
        Self {
            authority: LowerRuntimeEvidenceAuthority::SignalFacade,
            basis_digest: "missing-signal-observation".to_string(),
            evidence_digest: evidence_digest.into(),
            retained_evidence_lookup_width: 0,
            stale_runtime_snapshot: false,
            missing_signal_observation: true,
            unsupported_capability: false,
        }
    }

    pub fn unsupported_capability(
        authority_label: impl Into<String>,
        evidence_digest: impl Into<String>,
    ) -> Self {
        Self {
            authority: LowerRuntimeEvidenceAuthority::Unsupported,
            basis_digest: authority_label.into(),
            evidence_digest: evidence_digest.into(),
            retained_evidence_lookup_width: 0,
            stale_runtime_snapshot: false,
            missing_signal_observation: false,
            unsupported_capability: true,
        }
    }

    fn new(
        authority: LowerRuntimeEvidenceAuthority,
        basis_digest: impl Into<String>,
        evidence_digest: impl Into<String>,
        retained_evidence_lookup_width: usize,
    ) -> Self {
        Self {
            authority,
            basis_digest: basis_digest.into(),
            evidence_digest: evidence_digest.into(),
            retained_evidence_lookup_width,
            stale_runtime_snapshot: false,
            missing_signal_observation: false,
            unsupported_capability: false,
        }
    }

    pub fn authority(&self) -> LowerRuntimeEvidenceAuthority {
        self.authority
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn retained_evidence_lookup_width(&self) -> usize {
        self.retained_evidence_lookup_width
    }

    fn binding_digest(&self) -> String {
        hash_parts(&[
            "lower_runtime_readmission_v1".to_string(),
            format!("authority:{}", self.authority.as_str()),
            format!("basis:{}", self.basis_digest),
            format!("evidence:{}", self.evidence_digest),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundBasis<S: ScopedBasisProof> {
    scoped_basis: S,
    authority: LowerRuntimeEvidenceAuthority,
    basis_digest: String,
    evidence_digest: String,
    lower_runtime_binding_digest: String,
    readmission_trace: BasisEligibilityDecisionTrace,
    counters: BasisEligibilityCounters,
}

impl<S: ScopedBasisProof> LowerRuntimeBoundBasis<S> {
    pub(crate) fn new(scoped_basis: S, evidence: LowerRuntimeBasisEvidence) -> Self {
        let readmission_trace = BasisEligibilityDecisionTrace::new_lower_runtime_readmission(
            scoped_basis.scoped_basis_digest(),
            evidence.evidence_digest(),
            "success",
            "lower-runtime evidence readmitted into scoped Query basis",
        );
        Self {
            scoped_basis,
            authority: evidence.authority(),
            basis_digest: evidence.basis_digest().to_string(),
            evidence_digest: evidence.evidence_digest().to_string(),
            lower_runtime_binding_digest: evidence.binding_digest(),
            readmission_trace,
            counters: BasisEligibilityCounters::lower_runtime_readmission(
                0,
                evidence.retained_evidence_lookup_width(),
            ),
        }
    }

    pub fn scoped_basis(&self) -> &S {
        &self.scoped_basis
    }

    pub fn authority(&self) -> LowerRuntimeEvidenceAuthority {
        self.authority
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn lower_runtime_binding_digest(&self) -> &str {
        &self.lower_runtime_binding_digest
    }

    pub fn readmission_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.readmission_trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

pub fn readmit_lower_runtime_evidence<S: ScopedBasisProof>(
    scoped_basis: S,
    evidence: LowerRuntimeBasisEvidence,
) -> Result<LowerRuntimeBoundBasis<S>, DeniedBasisCapability> {
    if evidence.unsupported_capability {
        return Err(readmission_denial(
            DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported,
            &scoped_basis,
            &evidence,
            "lower-runtime capability is unsupported by the owning facade",
        ));
    }

    if evidence.missing_signal_observation {
        return Err(readmission_denial(
            DeniedBasisCapabilityKind::SignalObservationMissing,
            &scoped_basis,
            &evidence,
            "signal observation basis is required before readmission can bind",
        ));
    }

    if evidence.stale_runtime_snapshot {
        return Err(readmission_denial(
            DeniedBasisCapabilityKind::RuntimeSnapshotStale,
            &scoped_basis,
            &evidence,
            "runtime snapshot evidence is stale at the readmission boundary",
        ));
    }

    if !authority_matches_scoped_basis(&scoped_basis, &evidence) {
        return Err(readmission_denial(
            mismatch_denial_kind(&evidence),
            &scoped_basis,
            &evidence,
            "lower-runtime authority does not match the admitted Query basis",
        ));
    }

    if scoped_basis
        .expected_lower_runtime_binding_digest()
        .is_some_and(|expected| expected != evidence.basis_digest())
    {
        return Err(readmission_denial(
            mismatch_denial_kind(&evidence),
            &scoped_basis,
            &evidence,
            "lower-runtime basis digest does not match the admitted Query basis",
        ));
    }

    Ok(LowerRuntimeBoundBasis::new(scoped_basis, evidence))
}

fn authority_matches_scoped_basis<S: ScopedBasisProof>(
    scoped_basis: &S,
    evidence: &LowerRuntimeBasisEvidence,
) -> bool {
    evidence
        .authority()
        .basis_authority_posture()
        .is_some_and(|authority| authority == scoped_basis.authority())
}

fn mismatch_denial_kind(evidence: &LowerRuntimeBasisEvidence) -> DeniedBasisCapabilityKind {
    match evidence.authority() {
        LowerRuntimeEvidenceAuthority::RuntimeBridgeFacade => {
            DeniedBasisCapabilityKind::BridgeAuthorityMismatch
        }
        LowerRuntimeEvidenceAuthority::RelationalFacade => {
            DeniedBasisCapabilityKind::RelationalAuthorityMismatch
        }
        LowerRuntimeEvidenceAuthority::SignalFacade => {
            DeniedBasisCapabilityKind::SignalObservationMissing
        }
        LowerRuntimeEvidenceAuthority::Runtime | LowerRuntimeEvidenceAuthority::Unsupported => {
            DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported
        }
    }
}

fn readmission_denial<S: ScopedBasisProof>(
    kind: DeniedBasisCapabilityKind,
    scoped_basis: &S,
    evidence: &LowerRuntimeBasisEvidence,
    message: &'static str,
) -> DeniedBasisCapability {
    DeniedBasisCapability::new_readmission(
        kind,
        BasisEligibilityDecisionTrace::new_lower_runtime_readmission(
            scoped_basis.scoped_basis_digest(),
            evidence.evidence_digest(),
            "violation",
            message,
        ),
        BasisEligibilityCounters::lower_runtime_readmission(
            1,
            evidence.retained_evidence_lookup_width(),
        ),
    )
}

#[cfg(test)]
mod tests;
