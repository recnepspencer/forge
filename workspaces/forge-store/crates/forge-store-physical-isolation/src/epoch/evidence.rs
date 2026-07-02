use forge_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact,
    CanonicalBasisValue, CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use forge_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness, BasisPostureKind,
    CurrentValidity, FreshnessScopedBasis, NoProofs, RebindRequiredBasis, StaleReadableBasis,
    TransitionOutcome,
};

use super::{
    EpochRetryDecision, EpochStabilityScopeKind, PhysicalEpochDriftKind, PhysicalEpochFreshness,
    PhysicalEpochVector,
};

#[derive(Debug, Clone)]
pub struct PhysicalEpochFreshnessBasis {
    expected: PhysicalEpochVector,
    observed: PhysicalEpochVector,
    freshness: PhysicalEpochFreshness,
}

#[derive(Debug, Clone)]
pub struct PhysicalEpochComparisonEvidence {
    freshness: PhysicalEpochFreshness,
    foundational_basis: CanonicalBasisReadyArtifact,
    proof_evidence: PhysicalEpochFreshnessProofEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalEpochComparisonEvidenceDenial {
    FoundationalBasisDenied(CanonicalBasisConstructionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalEpochFreshnessProofPhase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PhysicalEpochFreshnessAuthority {
    _private: (),
}

impl AuthorityMarker for PhysicalEpochFreshnessAuthority {}

pub type PhysicalEpochFreshnessProofArtifact = Artifact<
    PhysicalEpochFreshnessProofPhase,
    PhysicalEpochFreshness,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<PhysicalEpochFreshnessBasis>>,
>;

pub type PhysicalEpochStaleReadableProofArtifact = Artifact<
    PhysicalEpochFreshnessProofPhase,
    PhysicalEpochFreshness,
    NoProofs,
    StaleReadableBasis<PhysicalEpochFreshnessBasis>,
>;

pub type PhysicalEpochRebindRequiredProofArtifact = Artifact<
    PhysicalEpochFreshnessProofPhase,
    PhysicalEpochFreshness,
    NoProofs,
    RebindRequiredBasis<PhysicalEpochFreshnessBasis>,
>;

#[derive(Debug, Clone)]
pub enum PhysicalEpochFreshnessProofEvidence {
    Current(PhysicalEpochFreshnessProofArtifact),
    StaleReadable(PhysicalEpochStaleReadableProofArtifact),
    RebindRequired(PhysicalEpochRebindRequiredProofArtifact),
}

pub fn compare_physical_epoch_vectors_with_evidence(
    expected: PhysicalEpochVector,
    observed: PhysicalEpochVector,
) -> Result<PhysicalEpochComparisonEvidence, PhysicalEpochComparisonEvidenceDenial> {
    let freshness = expected.compare_against(observed);
    let basis = PhysicalEpochFreshnessBasis {
        expected,
        observed,
        freshness,
    };
    let foundational_basis = prepare_foundational_basis(&basis)?;
    let authority =
        AuthorityWitness::from_authority_marker(PhysicalEpochFreshnessAuthority { _private: () });
    let current_proof = Artifact::with_current_basis(freshness, basis, authority);
    let proof_evidence = proof_evidence_for_decision(freshness.decision(), current_proof);

    Ok(PhysicalEpochComparisonEvidence {
        freshness,
        foundational_basis,
        proof_evidence,
    })
}

impl PhysicalEpochFreshnessBasis {
    pub const fn expected(&self) -> PhysicalEpochVector {
        self.expected
    }

    pub const fn observed(&self) -> PhysicalEpochVector {
        self.observed
    }

    pub const fn freshness(&self) -> PhysicalEpochFreshness {
        self.freshness
    }
}

impl PhysicalEpochComparisonEvidence {
    pub const fn freshness(&self) -> PhysicalEpochFreshness {
        self.freshness
    }

    pub const fn foundational_basis(&self) -> &CanonicalBasisReadyArtifact {
        &self.foundational_basis
    }

    pub const fn proof_evidence(&self) -> &PhysicalEpochFreshnessProofEvidence {
        &self.proof_evidence
    }
}

impl PhysicalEpochFreshnessProofEvidence {
    pub fn freshness(&self) -> &PhysicalEpochFreshness {
        match self {
            Self::Current(artifact) => artifact.payload(),
            Self::StaleReadable(artifact) => artifact.payload(),
            Self::RebindRequired(artifact) => artifact.payload(),
        }
    }

    pub fn basis(&self) -> &PhysicalEpochFreshnessBasis {
        match self {
            Self::Current(artifact) => artifact.strong_basis().value(),
            Self::StaleReadable(artifact) => artifact.basis().basis().value(),
            Self::RebindRequired(artifact) => artifact.basis().basis().value(),
        }
    }

    pub const fn basis_posture(&self) -> BasisPostureKind {
        match self {
            Self::Current(_) => BasisPostureKind::CurrentValidity,
            Self::StaleReadable(_) => BasisPostureKind::StaleReadable,
            Self::RebindRequired(_) => BasisPostureKind::RebindRequired,
        }
    }
}

fn proof_evidence_for_decision(
    decision: EpochRetryDecision,
    current_proof: PhysicalEpochFreshnessProofArtifact,
) -> PhysicalEpochFreshnessProofEvidence {
    match decision {
        EpochRetryDecision::Current => PhysicalEpochFreshnessProofEvidence::Current(current_proof),
        EpochRetryDecision::Retry => PhysicalEpochFreshnessProofEvidence::StaleReadable(
            current_proof.downgrade_to_stale_readable(),
        ),
        EpochRetryDecision::RebindRequired => PhysicalEpochFreshnessProofEvidence::RebindRequired(
            current_proof.downgrade_to_rebind_required(),
        ),
    }
}

fn prepare_foundational_basis(
    basis: &PhysicalEpochFreshnessBasis,
) -> Result<CanonicalBasisReadyArtifact, PhysicalEpochComparisonEvidenceDenial> {
    match prepare_canonical_basis_sequence(rule_version(), epoch_domain(), canonical_entries(basis))
    {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => {
            Err(PhysicalEpochComparisonEvidenceDenial::FoundationalBasisDenied(denial))
        }
        _ => unreachable!("canonical basis preparation only succeeds or denies"),
    }
}

fn canonical_entries(basis: &PhysicalEpochFreshnessBasis) -> Vec<CanonicalBasisEntry> {
    let expected = basis.expected();
    let observed = basis.observed();
    vec![
        text_entry("decision", decision_label(basis.freshness().decision())),
        text_entry("drift", drift_label(basis.freshness().drift())),
        text_entry("expected.scope.kind", scope_label(expected.scope().kind())),
        u64_entry("expected.scope.root", expected.scope().root_scope_id()),
        u64_entry("expected.root", expected.root_epoch().get()),
        u64_entry("expected.manifest", expected.manifest_epoch().get()),
        option_entry(
            "expected.segment",
            expected.segment_epoch().map(|epoch| epoch.get()),
        ),
        option_entry(
            "expected.extent",
            expected.extent_epoch().map(|epoch| epoch.get()),
        ),
        option_entry(
            "expected.page",
            expected.page_epoch().map(|epoch| epoch.get()),
        ),
        option_entry(
            "expected.chunk",
            expected.chunk_epoch().map(|epoch| epoch.get()),
        ),
        text_entry("observed.scope.kind", scope_label(observed.scope().kind())),
        u64_entry("observed.scope.root", observed.scope().root_scope_id()),
        u64_entry("observed.root", observed.root_epoch().get()),
        u64_entry("observed.manifest", observed.manifest_epoch().get()),
        option_entry(
            "observed.segment",
            observed.segment_epoch().map(|epoch| epoch.get()),
        ),
        option_entry(
            "observed.extent",
            observed.extent_epoch().map(|epoch| epoch.get()),
        ),
        option_entry(
            "observed.page",
            observed.page_epoch().map(|epoch| epoch.get()),
        ),
        option_entry(
            "observed.chunk",
            observed.chunk_epoch().map(|epoch| epoch.get()),
        ),
    ]
}

fn text_entry(locus: &'static str, value: &'static str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        epoch_domain(),
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Future("s5-physical-epoch-freshness"),
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn u64_entry(locus: &'static str, value: u64) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        epoch_domain(),
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Future("s5-physical-epoch-freshness"),
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value.into(),
        },
    )
}

fn option_entry(locus: &'static str, value: Option<u64>) -> CanonicalBasisEntry {
    match value {
        Some(value) => u64_entry(locus, value),
        None => text_entry(locus, "none"),
    }
}

fn decision_label(decision: EpochRetryDecision) -> &'static str {
    match decision {
        EpochRetryDecision::Current => "current",
        EpochRetryDecision::Retry => "retry",
        EpochRetryDecision::RebindRequired => "rebind-required",
    }
}

fn drift_label(drift: Option<PhysicalEpochDriftKind>) -> &'static str {
    match drift {
        None => "none",
        Some(PhysicalEpochDriftKind::ScopeMismatch) => "scope-mismatch",
        Some(PhysicalEpochDriftKind::RootEpoch) => "root-epoch",
        Some(PhysicalEpochDriftKind::ManifestEpoch) => "manifest-epoch",
        Some(PhysicalEpochDriftKind::SegmentEpoch) => "segment-epoch",
        Some(PhysicalEpochDriftKind::ExtentEpoch) => "extent-epoch",
        Some(PhysicalEpochDriftKind::PageEpoch) => "page-epoch",
        Some(PhysicalEpochDriftKind::ChunkEpoch) => "chunk-epoch",
    }
}

fn scope_label(scope: EpochStabilityScopeKind) -> &'static str {
    match scope {
        EpochStabilityScopeKind::ReadPlanAdmission => "read-plan-admission",
        EpochStabilityScopeKind::RootReadmission => "root-readmission",
        EpochStabilityScopeKind::ReferenceValidation => "reference-validation",
    }
}

fn rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("forge-store.s5.physical-epoch-freshness.v1")
        .expect("static physical epoch freshness rule version is valid")
}

const fn epoch_domain() -> CanonicalBasisDomain {
    CanonicalBasisDomain::Future("forge-store.s5.physical-epoch-freshness")
}
