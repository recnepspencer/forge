use super::production_phase_two_closeout;
use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPlanAdoptionAttemptKind, WorthGraphReadAccessPlanAdoptionPostureKind,
    QUERY_ACCESS_POSTURE_MATRIX,
};
use forge_query::facade::ForgeQueryGraphReadAccessAdmissionPosture;

#[test]
fn current_phase_two_attempts_expose_missing_query_read_family_artifact() {
    let closeout = production_phase_two_closeout();

    assert_eq!(closeout.counters().query_admission_inspected_count(), 0);
    assert_eq!(closeout.counters().admitted_plan_count(), 0);
    assert_eq!(
        closeout.counters().required_or_denied_posture_count(),
        closeout.counters().adoption_attempt_count()
    );
    assert_eq!(
        closeout
            .counters()
            .missing_query_read_family_artifact_count(),
        closeout.counters().adoption_attempt_count()
    );
    assert!(closeout.adoption_ledger().attempts().iter().all(|attempt| {
        attempt.kind()
            == WorthGraphReadAccessPlanAdoptionAttemptKind::MissingQueryReadFamilyArtifact
            && attempt.query_admission_digest().is_none()
            && attempt.query_requirement_set_digest().is_none()
            && attempt.admitted_plan_digest().is_none()
            && attempt.query_posture() == Some("query_read_family_artifact_required")
            && attempt.denial_kind() == Some("missing_query_read_family_artifact")
    }));
}

#[test]
fn phase_two_exposes_access_posture_report_as_control_plane_product() {
    let closeout = production_phase_two_closeout();

    assert_eq!(
        closeout.posture_report().posture_rows().len(),
        closeout.adoption_ledger().attempts().len()
    );
    assert_eq!(
        closeout.posture_report().required_or_denied_posture_count(),
        closeout.posture_report().posture_rows().len()
    );
    assert_eq!(
        closeout
            .posture_report()
            .missing_query_read_family_artifact_count(),
        closeout.posture_report().posture_rows().len()
    );
    assert!(closeout.posture_report().posture_rows().iter().all(|row| {
        row.posture_kind()
            == WorthGraphReadAccessPlanAdoptionPostureKind::MissingQueryReadFamilyArtifact
            && row.query_posture() == "query_read_family_artifact_required"
            && row.denial_kind() == Some("missing_query_read_family_artifact")
            && !row.row_digest().is_empty()
    }));
}

#[test]
fn phase_two_names_real_query_api_required_for_admission() {
    let closeout = production_phase_two_closeout();

    assert!(closeout.adoption_ledger().attempts().iter().all(|attempt| {
        attempt
            .query_api_required()
            .contains("admit_graph_read_access_for_family_in_authority")
            && attempt.blocker().is_some()
            && attempt.removal_trigger().is_some()
    }));
}

#[test]
fn phase_two_posture_matrix_covers_query_access_posture_vocabulary() {
    let worth_postures = QUERY_ACCESS_POSTURE_MATRIX
        .iter()
        .map(|posture| posture.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let query_postures = ForgeQueryGraphReadAccessAdmissionPosture::ALL
        .iter()
        .map(ForgeQueryGraphReadAccessAdmissionPosture::as_str)
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(
        worth_postures, query_postures,
        "Phase 2 must be able to represent every Query access posture before execution"
    );
}
