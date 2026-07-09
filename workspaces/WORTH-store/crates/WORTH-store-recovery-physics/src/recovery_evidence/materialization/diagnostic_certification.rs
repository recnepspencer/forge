use worth_foundational::{
    admit_current_basis_boundary_bundle, bridge_certified_diagnostic_bundle_trust_boundary,
    certify_current_basis_diagnostic_bundle, foundational_boundary_current_basis_authority,
    foundational_diagnostic_certified_attachment_authority,
    foundational_diagnostic_certified_readmission_authority,
    readmit_certified_diagnostic_bundle_after_boundary, CanonicalizationRuleVersion,
    FoundationalDiagnosticCoverageFamilyStatus, FoundationalDiagnosticCoverageMatrix,
    FoundationalDiagnosticRow, FoundationalDiagnosticRowFamily,
};
use worth_proof::TransitionOutcome;

use super::super::denial::RecoveryEvidenceDenial;
use super::super::diagnostics::RecoverySourceDecisionReport;
use super::foundational_bundle::{
    MaterializedFoundationalRecoveryEvidenceBundle, RecoveryCertifiedDiagnosticSupportBundle,
    RecoveryCurrentBasisBoundaryBundle,
};

pub(crate) fn certify_diagnostic_support_bundle(
    materialized: &MaterializedFoundationalRecoveryEvidenceBundle,
    source_decisions: &RecoverySourceDecisionReport,
) -> Result<RecoveryCertifiedDiagnosticSupportBundle, RecoveryEvidenceDenial> {
    let current_basis = current_basis_boundary_bundle(materialized)?;
    match certify_current_basis_diagnostic_bundle(
        diagnostic_certification_version(),
        current_basis,
        source_decisions.support_report().clone(),
        diagnostic_coverage_matrix(source_decisions.foundational_rows()),
        foundational_diagnostic_certified_attachment_authority(),
    ) {
        TransitionOutcome::Success(bundle) => Ok(bundle),
        _ => Err(RecoveryEvidenceDenial::DiagnosticCertificationDenied),
    }
}

pub(crate) fn readmit_diagnostic_support_bundle(
    certified: RecoveryCertifiedDiagnosticSupportBundle,
) -> Result<RecoveryCertifiedDiagnosticSupportBundle, RecoveryEvidenceDenial> {
    let basis = certified.strong_basis().clone();
    let bridged = bridge_certified_diagnostic_bundle_trust_boundary(certified);
    Ok(readmit_certified_diagnostic_bundle_after_boundary(
        bridged,
        basis,
        foundational_diagnostic_certified_readmission_authority(),
    ))
}

fn current_basis_boundary_bundle(
    materialized: &MaterializedFoundationalRecoveryEvidenceBundle,
) -> Result<RecoveryCurrentBasisBoundaryBundle, RecoveryEvidenceDenial> {
    match admit_current_basis_boundary_bundle(
        diagnostic_certification_version(),
        materialized.clone(),
        foundational_boundary_current_basis_authority(),
    ) {
        TransitionOutcome::Success(bundle) => Ok(bundle),
        _ => Err(RecoveryEvidenceDenial::CurrentBasisAdmissionDenied),
    }
}

fn diagnostic_certification_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.s4.recovery.diagnostic-certification")
        .expect("static diagnostic certification version")
}

fn diagnostic_coverage_matrix(
    rows: &[FoundationalDiagnosticRow],
) -> FoundationalDiagnosticCoverageMatrix {
    FoundationalDiagnosticCoverageMatrix::new(
        coverage_status(rows, FoundationalDiagnosticRowFamily::Decision),
        coverage_status(rows, FoundationalDiagnosticRowFamily::Failure),
        coverage_status(rows, FoundationalDiagnosticRowFamily::Comparison),
        coverage_status(rows, FoundationalDiagnosticRowFamily::Support),
        coverage_status(rows, FoundationalDiagnosticRowFamily::ProvenanceReady),
    )
}

fn coverage_status(
    rows: &[FoundationalDiagnosticRow],
    family: FoundationalDiagnosticRowFamily,
) -> FoundationalDiagnosticCoverageFamilyStatus {
    let row_count = rows.iter().filter(|row| row.family() == family).count() as u32;
    if row_count == 0 {
        FoundationalDiagnosticCoverageFamilyStatus::AbsentFromBundle
    } else {
        FoundationalDiagnosticCoverageFamilyStatus::HostileRowsPresent { row_count }
    }
}
