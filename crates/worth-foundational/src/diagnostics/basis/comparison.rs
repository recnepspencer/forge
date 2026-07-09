use worth_proof::TransitionOutcome;

use crate::canonicalization::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalBasisConstructionDenial,
    CanonicalComparisonOutcome, CanonicalEquivalenceBasis, CanonicalMismatchBasis,
    CanonicalizationRuleVersion,
};
use crate::diagnostics::{
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticSubject, FoundationalDiagnosticSupportReport,
};

use super::canonical::{
    prepare_diagnostic_explanation_bundle_for_canonical_basis,
    prepare_diagnostic_support_report_for_canonical_basis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticComparisonDenial {
    LeftBasisConstructionDenied(CanonicalBasisConstructionDenial),
    RightBasisConstructionDenied(CanonicalBasisConstructionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticComparisonBundle {
    left_artifact_kind: FoundationalDiagnosticArtifactKind,
    right_artifact_kind: FoundationalDiagnosticArtifactKind,
    left_subject: FoundationalDiagnosticSubject,
    right_subject: FoundationalDiagnosticSubject,
    left_row_count: u32,
    right_row_count: u32,
    outcome: CanonicalComparisonOutcome,
}

impl FoundationalDiagnosticComparisonBundle {
    pub const fn left_artifact_kind(&self) -> FoundationalDiagnosticArtifactKind {
        self.left_artifact_kind
    }

    pub const fn right_artifact_kind(&self) -> FoundationalDiagnosticArtifactKind {
        self.right_artifact_kind
    }

    pub fn left_subject(&self) -> &FoundationalDiagnosticSubject {
        &self.left_subject
    }

    pub fn right_subject(&self) -> &FoundationalDiagnosticSubject {
        &self.right_subject
    }

    pub const fn left_row_count(&self) -> u32 {
        self.left_row_count
    }

    pub const fn right_row_count(&self) -> u32 {
        self.right_row_count
    }

    pub fn outcome(&self) -> &CanonicalComparisonOutcome {
        &self.outcome
    }

    pub fn mismatch_basis(&self) -> Option<&CanonicalMismatchBasis> {
        match &self.outcome {
            CanonicalComparisonOutcome::Equivalent(_) => None,
            CanonicalComparisonOutcome::Mismatched(basis)
            | CanonicalComparisonOutcome::Unsupported(basis) => Some(basis),
        }
    }
}

pub fn compare_diagnostic_support_reports(
    version: CanonicalizationRuleVersion,
    left: &FoundationalDiagnosticSupportReport,
    right: &FoundationalDiagnosticSupportReport,
) -> TransitionOutcome<FoundationalDiagnosticComparisonBundle, FoundationalDiagnosticComparisonDenial>
{
    compare_diagnostic_surfaces(
        FoundationalDiagnosticArtifactKind::SupportReport,
        left.subject().clone(),
        left.rows().len() as u32,
        FoundationalDiagnosticArtifactKind::SupportReport,
        right.subject().clone(),
        right.rows().len() as u32,
        prepare_diagnostic_support_report_for_canonical_basis(version.clone(), left),
        prepare_diagnostic_support_report_for_canonical_basis(version, right),
    )
}

pub fn compare_diagnostic_explanation_bundles(
    version: CanonicalizationRuleVersion,
    left: &FoundationalDiagnosticExplanationBundle,
    right: &FoundationalDiagnosticExplanationBundle,
) -> TransitionOutcome<FoundationalDiagnosticComparisonBundle, FoundationalDiagnosticComparisonDenial>
{
    compare_diagnostic_surfaces(
        FoundationalDiagnosticArtifactKind::ExplanationBundle,
        left.subject().clone(),
        left.rows().len() as u32,
        FoundationalDiagnosticArtifactKind::ExplanationBundle,
        right.subject().clone(),
        right.rows().len() as u32,
        prepare_diagnostic_explanation_bundle_for_canonical_basis(version.clone(), left),
        prepare_diagnostic_explanation_bundle_for_canonical_basis(version, right),
    )
}

fn compare_diagnostic_surfaces(
    left_artifact_kind: FoundationalDiagnosticArtifactKind,
    left_subject: FoundationalDiagnosticSubject,
    left_row_count: u32,
    right_artifact_kind: FoundationalDiagnosticArtifactKind,
    right_subject: FoundationalDiagnosticSubject,
    right_row_count: u32,
    left_ready: TransitionOutcome<
        crate::canonicalization::CanonicalBasisReadyArtifact,
        CanonicalBasisConstructionDenial,
    >,
    right_ready: TransitionOutcome<
        crate::canonicalization::CanonicalBasisReadyArtifact,
        CanonicalBasisConstructionDenial,
    >,
) -> TransitionOutcome<FoundationalDiagnosticComparisonBundle, FoundationalDiagnosticComparisonDenial>
{
    let left_ready = match left_ready {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            return TransitionOutcome::denied(
                FoundationalDiagnosticComparisonDenial::LeftBasisConstructionDenied(denial),
            );
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("diagnostic basis preparation uses only denied")
        }
    };
    let right_ready = match right_ready {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            return TransitionOutcome::denied(
                FoundationalDiagnosticComparisonDenial::RightBasisConstructionDenied(denial),
            );
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("diagnostic basis preparation uses only denied")
        }
    };
    let ready = match prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        left_ready,
        right_ready,
    ) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(_)
        | TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("canonical comparison readiness is infallible")
        }
    };

    TransitionOutcome::success(FoundationalDiagnosticComparisonBundle {
        left_artifact_kind,
        right_artifact_kind,
        left_subject,
        right_subject,
        left_row_count,
        right_row_count,
        outcome: compare_canonical_basis(&ready),
    })
}
