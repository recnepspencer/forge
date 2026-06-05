use forge_foundational::facade::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact,
    CanonicalBasisValue, CanonicalizationRuleVersion,
};
use forge_proof::TransitionOutcome;

use super::{SignalBranchBasis, SignalBranchBasisDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalBranchBasisCompactExplanation {
    summary: String,
    labels: Vec<&'static str>,
}

impl SignalBranchBasisCompactExplanation {
    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn labels(&self) -> &[&'static str] {
        &self.labels
    }
}

impl SignalBranchBasis {
    pub fn prepare_canonical_basis(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_canonical_basis_sequence(
            version,
            CanonicalBasisDomain::Transition,
            branch_basis_entries(self),
        )
    }

    pub fn prepare_locator_for_canonical_basis(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_canonical_basis_sequence(
            version,
            CanonicalBasisDomain::Locator,
            branch_basis_locator_entries(self),
        )
    }

    pub fn compact_explanation(&self) -> SignalBranchBasisCompactExplanation {
        SignalBranchBasisCompactExplanation {
            summary: format!(
                "branch basis for {} at {:?}",
                self.branch_name(),
                self.head_posture()
            ),
            labels: vec!["branch-basis", "retained", restore_label(self)],
        }
    }
}

impl SignalBranchBasisDenial {
    pub fn prepare_canonical_basis(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_canonical_basis_sequence(
            version,
            CanonicalBasisDomain::Transition,
            branch_basis_denial_entries(self),
        )
    }
}

fn branch_basis_entries(basis: &SignalBranchBasis) -> Vec<CanonicalBasisEntry> {
    vec![
        text_entry("branch-basis.schema-version", basis.schema_version()),
        text_entry("branch-basis.branch-id", basis.branch_id().0.to_string()),
        text_entry("branch-basis.branch-name", basis.branch_name().to_owned()),
        text_entry(
            "branch-basis.snapshot-id",
            format!("{:?}", basis.snapshot_id().map(|snapshot| snapshot.0)),
        ),
        text_entry(
            "branch-basis.head-posture",
            format!("{:?}", basis.head_posture()),
        ),
        text_entry(
            "branch-basis.restore-posture",
            format!("{:?}", basis.restore_posture()),
        ),
        text_entry(
            "branch-basis.branch-component-digest",
            basis.branch_component_digest().to_owned(),
        ),
        text_entry(
            "branch-basis.snapshot-component-digest",
            basis.snapshot_component_digest().to_owned(),
        ),
        text_entry(
            "branch-basis.head-component-digest",
            basis.head_component_digest().to_owned(),
        ),
        text_entry(
            "branch-basis.restore-component-digest",
            basis.restore_component_digest().to_owned(),
        ),
        text_entry("branch-basis.basis-digest", basis.basis_digest().to_owned()),
    ]
}

fn branch_basis_locator_entries(basis: &SignalBranchBasis) -> Vec<CanonicalBasisEntry> {
    vec![
        locator_entry("branch-basis", "signal-branch-basis"),
        locator_entry("branch-basis.branch-id", basis.branch_id().0.to_string()),
        locator_entry(
            "branch-basis.snapshot-id",
            format!("{:?}", basis.snapshot_id().map(|snapshot| snapshot.0)),
        ),
        locator_entry(
            "branch-basis.head-posture",
            format!("{:?}", basis.head_posture()),
        ),
        locator_entry(
            "branch-basis.restore-posture",
            format!("{:?}", basis.restore_posture()),
        ),
    ]
}

fn branch_basis_denial_entries(denial: &SignalBranchBasisDenial) -> Vec<CanonicalBasisEntry> {
    vec![text_entry("branch-basis.denial", format!("{denial:?}"))]
}

fn restore_label(basis: &SignalBranchBasis) -> &'static str {
    match basis.restore_posture() {
        super::SignalBranchRestorePosture::NotRestoreDerived => "live",
        super::SignalBranchRestorePosture::SnapshotRestore { .. } => "restore-derived",
    }
}

fn text_entry(name: &'static str, value: impl Into<String>) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Transition,
        CanonicalBasisLocus::Named(name.into()),
        CanonicalBasisEntryKind::TransitionArtifact,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}

fn locator_entry(name: &'static str, value: impl Into<String>) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(name.into()),
        CanonicalBasisEntryKind::Locator,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}
