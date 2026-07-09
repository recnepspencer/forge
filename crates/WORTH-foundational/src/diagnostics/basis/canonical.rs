use worth_proof::TransitionOutcome;

use crate::canonicalization::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use crate::diagnostics::{
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticSupportReport,
};

use super::entries::diagnostic_bundle_entries;

pub fn prepare_diagnostic_support_report_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    report: &FoundationalDiagnosticSupportReport,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Diagnostic,
        diagnostic_bundle_entries(
            FoundationalDiagnosticArtifactKind::SupportReport,
            report.subject(),
            report.outcome_kind(),
            report.profile(),
            report.delivery_class(),
            report.availability(),
            report.partiality(),
            report.counter_snapshot(),
            report.assembly_debts(),
            Some(report.support_claim_strength()),
            report.rows(),
        ),
    )
}

pub fn prepare_diagnostic_explanation_bundle_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    bundle: &FoundationalDiagnosticExplanationBundle,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Diagnostic,
        diagnostic_bundle_entries(
            FoundationalDiagnosticArtifactKind::ExplanationBundle,
            bundle.subject(),
            bundle.outcome_kind(),
            bundle.profile(),
            bundle.delivery_class(),
            bundle.availability(),
            bundle.partiality(),
            bundle.counter_snapshot(),
            bundle.assembly_debts(),
            None,
            bundle.rows(),
        ),
    )
}

pub fn foundational_diagnostic_canonical_basis_entries(
    ready: &CanonicalBasisReadyArtifact,
) -> &[CanonicalBasisEntry] {
    ready.payload().entries()
}
