mod admission_payload_bytes;
mod census_foundation;
mod execution_lifecycle;

use worth_ui::facade::intent::UiIntentResourceCensus;
use worth_ui_runtime::certification_support::{
    WorthUiIntentEvidenceCertificationExt, WorthUiIntentResourceCensusCertificationExt,
};

pub(super) fn census(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> UiIntentResourceCensus {
    session.intent_resource_census_for_certification()
}

pub(super) fn assert_empty(census: UiIntentResourceCensus) {
    assert_eq!(census, UiIntentResourceCensus::EMPTY);
    assert!(census.is_empty());
}

pub(super) fn assert_evidence_count(census: UiIntentResourceCensus, expected: usize) {
    assert_eq!(census.retained_evidence_references(), expected);
    assert_eq!(
        census.retained_evidence_bytes(),
        expected * core::mem::size_of::<worth_ui_inspection::UiIntentCausalTraceEvidence>()
    );
}

pub(super) fn assert_only_evidence(census: UiIntentResourceCensus, expected: usize) {
    assert_evidence_count(census, expected);
    assert!(census.is_operationally_empty());
    assert_eq!(census.is_empty(), expected == 0);
}

pub(super) fn latest_evidence(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui_inspection::UiIntentInteractionEvidence {
    let reference = latest_evidence_reference(session);
    session
        .lookup_intent_evidence_for_certification(reference)
        .expect("the owner resolves its exact live evidence reference")
}

pub(super) fn latest_evidence_reference(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui_inspection::UiIntentEvidenceReference {
    session
        .latest_intent_evidence_reference_for_certification()
        .expect("one semantic interaction retains an evidence reference")
}

pub(super) fn lookup_evidence(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    reference: worth_ui_inspection::UiIntentEvidenceReference,
) -> Option<worth_ui_inspection::UiIntentInteractionEvidence> {
    session.lookup_intent_evidence_for_certification(reference)
}

pub(super) fn assert_retirement(
    report: worth_ui_inspection::UiIntentEvidenceRetirementReport,
    cause: worth_ui_inspection::UiIntentEvidenceRetirementCause,
    expected: usize,
) {
    assert_eq!(report.cause(), cause);
    assert_eq!(report.disposed_references(), expected);
    assert_eq!(
        report.disposed_bytes(),
        expected * core::mem::size_of::<worth_ui_inspection::UiIntentCausalTraceEvidence>()
    );
    assert_eq!(report.active_after(), 0);
}

pub(super) fn assert_observation_retirement(
    report: worth_ui::facade::observation::UiObservationResourceRetirementReport,
    cause: worth_ui::facade::observation::UiObservationResourceRetirementCause,
    expected_sets: usize,
    expected_observations: usize,
    expected_bytes: usize,
) {
    assert_eq!(report.cause(), cause);
    assert_eq!(report.disposed_sets(), expected_sets);
    assert_eq!(report.disposed_observations(), expected_observations);
    assert_eq!(report.disposed_bytes(), expected_bytes);
    assert_eq!(report.active_after(), 0);
}
