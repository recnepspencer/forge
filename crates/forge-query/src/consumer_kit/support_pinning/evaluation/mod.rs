mod finding;
mod report;

use super::super::support_snapshot::{ForgeQuerySupportSnapshot, ForgeQuerySupportSnapshotRow};
use super::contract::ForgeQuerySupportPinContract;
use super::document::schema::support_pin_vocabulary_identity;
use super::error::ForgeQuerySupportPinningError;
use super::evidence::{derive_support_pin_finding_identity, derive_support_pin_report_identity};
use super::observed_row::ForgeQueryObservedSupportPin;
use super::requirement::ForgeQuerySupportPinRequirement;
use super::snapshot_index::SupportPinSnapshotIndex;

pub use finding::{ForgeQuerySupportPinFinding, ForgeQuerySupportPinFindingKind};
pub use report::ForgeQuerySupportPinReport;

pub(crate) fn evaluate_support_pin_contract(
    contract: &ForgeQuerySupportPinContract,
    snapshot: &ForgeQuerySupportSnapshot,
) -> Result<ForgeQuerySupportPinReport, ForgeQuerySupportPinningError> {
    let index = SupportPinSnapshotIndex::new(snapshot)?;
    let mut findings = Vec::new();
    let mut matched_required_count = 0;

    if contract.schema_identity() != snapshot.schema_identity() {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::SchemaMismatch,
            None,
            "support-snapshot-schema",
            Some(contract.schema_identity().to_string()),
            Some(snapshot.schema_identity().to_string()),
            true,
        ));
    }

    let expected_vocabulary = support_pin_vocabulary_identity()
        .terminal_projection_for_reporting()
        .to_string();
    if contract.pinned_vocabulary_identity() != expected_vocabulary {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::VocabularyMismatch,
            None,
            "support-pin-vocabulary",
            Some(expected_vocabulary),
            Some(contract.pinned_vocabulary_identity().to_string()),
            true,
        ));
    }

    if contract.source_matrix_digest() != snapshot.source_matrix_digest() {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::SourceMatrixDigestChanged,
            None,
            "support-matrix",
            Some(contract.source_matrix_digest().to_string()),
            Some(snapshot.source_matrix_digest().to_string()),
            false,
        ));
    }

    for requirement in contract.requirements() {
        match index.optional_row(requirement.family()) {
            Some(row) => {
                matched_required_count += 1;
                findings.extend(evaluate_requirement(requirement, row));
            }
            None => findings.push(make_finding(
                ForgeQuerySupportPinFindingKind::RequiredRowMissing,
                Some(requirement.family()),
                requirement.surface(),
                Some(requirement.family().as_str().to_string()),
                None,
                true,
            )),
        }
    }

    for observed in contract.observed_rows() {
        match index.optional_row(observed.family()) {
            Some(row) => findings.extend(evaluate_observed_row(observed, row)),
            None => findings.push(make_finding(
                ForgeQuerySupportPinFindingKind::ObservedRowMissing,
                Some(observed.family()),
                observed.surface(),
                Some(observed.family().as_str().to_string()),
                None,
                false,
            )),
        }
    }

    let finding_identities = findings
        .iter()
        .map(derive_support_pin_finding_identity)
        .collect::<Vec<_>>();
    let report_digest = derive_support_pin_report_identity(
        contract.consumer_name(),
        contract.contract_digest(),
        snapshot.schema_identity(),
        snapshot.source_matrix_digest(),
        snapshot.snapshot_digest(),
        contract.requirements().len(),
        contract.observed_rows().len(),
        matched_required_count,
        snapshot.rows().len(),
        &finding_identities,
    )
    .terminal_projection_for_reporting()
    .to_string();

    Ok(ForgeQuerySupportPinReport::new(
        contract.consumer_name().to_string(),
        contract.contract_digest().to_string(),
        snapshot.schema_identity().to_string(),
        snapshot.source_matrix_digest().to_string(),
        snapshot.snapshot_digest().to_string(),
        contract.requirements().len(),
        contract.observed_rows().len(),
        matched_required_count,
        snapshot.rows().len(),
        findings,
        report_digest,
    ))
}

fn evaluate_requirement(
    requirement: &ForgeQuerySupportPinRequirement,
    row: &ForgeQuerySupportSnapshotRow,
) -> Vec<ForgeQuerySupportPinFinding> {
    let mut findings = Vec::new();
    if requirement.required_status().as_str() != row.status() {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::StatusMismatch,
            Some(requirement.family()),
            requirement.surface(),
            Some(requirement.required_status().as_str().to_string()),
            Some(row.status().to_string()),
            true,
        ));
    }
    if requirement.required_teaching_posture().as_str() != row.teaching_posture() {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::TeachingPostureMismatch,
            Some(requirement.family()),
            requirement.surface(),
            Some(requirement.required_teaching_posture().as_str().to_string()),
            Some(row.teaching_posture().to_string()),
            true,
        ));
    }
    if requirement.pinned_live_row_digest() != row.live_row_digest() {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::LiveRowDigestMismatch,
            Some(requirement.family()),
            requirement.surface(),
            Some(requirement.pinned_live_row_digest().to_string()),
            Some(row.live_row_digest().to_string()),
            true,
        ));
    }
    findings
}

fn evaluate_observed_row(
    observed: &ForgeQueryObservedSupportPin,
    row: &ForgeQuerySupportSnapshotRow,
) -> Vec<ForgeQuerySupportPinFinding> {
    let mut findings = Vec::new();
    if observed.observed_status() != row.status() {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::ObservedStatusChanged,
            Some(observed.family()),
            observed.surface(),
            Some(observed.observed_status().to_string()),
            Some(row.status().to_string()),
            false,
        ));
    }
    if observed.observed_teaching_posture() != row.teaching_posture() {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::ObservedTeachingPostureChanged,
            Some(observed.family()),
            observed.surface(),
            Some(observed.observed_teaching_posture().to_string()),
            Some(row.teaching_posture().to_string()),
            false,
        ));
    }
    if observed.observed_live_row_digest() != Some(row.live_row_digest()) {
        findings.push(make_finding(
            ForgeQuerySupportPinFindingKind::ObservedLiveRowDigestChanged,
            Some(observed.family()),
            observed.surface(),
            observed.observed_live_row_digest().map(str::to_string),
            Some(row.live_row_digest().to_string()),
            false,
        ));
    }
    findings
}

fn make_finding(
    kind: ForgeQuerySupportPinFindingKind,
    family: Option<crate::runtime::ForgeQueryRuntimeFacadeFamily>,
    surface: impl Into<String>,
    expected: Option<String>,
    found: Option<String>,
    blocking: bool,
) -> ForgeQuerySupportPinFinding {
    let surface = surface.into();
    let provisional = ForgeQuerySupportPinFinding::new(
        kind,
        family,
        surface.clone(),
        expected.clone(),
        found.clone(),
        blocking,
        String::new(),
    );
    let digest = derive_support_pin_finding_identity(&provisional)
        .terminal_projection_for_reporting()
        .to_string();
    ForgeQuerySupportPinFinding::new(kind, family, surface, expected, found, blocking, digest)
}
