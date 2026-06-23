use super::query_disabled_application_facade;
use crate::application::{
    identity_boundary_hostile_matrix_artifact, milestone_nine_six_certification_gate_certified,
    ForgeQueryApplicationFacade, ForgeQueryFolkloreResidueStatus, ForgeQueryMilestoneClosureStatus,
    EVIDENCE_IDENTITY_COVERED_SURFACES, EXACT_ZERO_FORMAT_DIGEST_PATHS,
    EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS, EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS,
    EXACT_ZERO_STRING_MATCHING_PATHS, MILESTONE_9_6_CERTIFICATION_GATE_PATHS,
    SESSION_LABEL_ORDINARY_ENTRYPOINTS, STOP_CLASS_COVERED_CONTRACTS,
};
use crate::ForgeQueryEvidenceIdentityScheme;

#[test]
fn support_report_derives_closed_identity_boundary_from_clean_runtime_backed_surface() {
    let report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = report.identity_boundary_closure();

    assert_eq!(closure.status(), ForgeQueryMilestoneClosureStatus::Closed);
    assert_eq!(
        closure.evidence_identity().status(),
        ForgeQueryMilestoneClosureStatus::Closed
    );
    assert_eq!(
        closure.evidence_identity().scheme(),
        ForgeQueryEvidenceIdentityScheme::V1
    );
    assert_eq!(
        closure.evidence_identity().covered_surfaces(),
        EVIDENCE_IDENTITY_COVERED_SURFACES
    );
    assert_eq!(
        closure.stop_class().status(),
        ForgeQueryMilestoneClosureStatus::Closed
    );
    assert_eq!(
        closure.stop_class().accessor(),
        "ForgeQueryRuntimeError::stop_class()"
    );
    assert_eq!(
        closure.stop_class().covered_contracts(),
        STOP_CLASS_COVERED_CONTRACTS
    );
    assert_eq!(
        closure.session_label().status(),
        ForgeQueryMilestoneClosureStatus::Closed
    );
    assert_eq!(
        closure.session_label().entry_label_type(),
        "ForgeQuerySessionLabel"
    );
    assert_eq!(
        closure.session_label().collision_stop_class(),
        "ForgeQueryStopClass::SessionLabelCollision"
    );
    assert_eq!(
        closure.session_label().ordinary_entrypoints(),
        SESSION_LABEL_ORDINARY_ENTRYPOINTS
    );
    assert!(closure.residue_status().is_zero());
    assert!(matches!(
        closure.residue_status(),
        ForgeQueryFolkloreResidueStatus::ZeroFolkloreResidue
    ));
    assert_eq!(
        closure.exact_zero_format_digest_paths(),
        EXACT_ZERO_FORMAT_DIGEST_PATHS
    );
    assert_eq!(
        closure.exact_zero_string_matching_paths(),
        EXACT_ZERO_STRING_MATCHING_PATHS
    );
    assert_eq!(
        closure.exact_zero_raw_session_admission_paths(),
        EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS
    );
    assert_eq!(
        closure.exact_zero_string_carried_session_identity_paths(),
        EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS
    );
    assert!(closure.hostile_matrix_certified());
    assert!(closure.certification_gate_certified());
    assert!(milestone_nine_six_certification_gate_certified());
    assert_eq!(
        MILESTONE_9_6_CERTIFICATION_GATE_PATHS.len(),
        4,
        "named Milestone 9.6 certification suites must remain registered"
    );
    assert!(!closure.hostile_matrix_digest().is_empty());
    assert_eq!(
        closure.evidence_identity().closure_identity().as_str(),
        closure.evidence_identity().closure_digest()
    );
    assert_eq!(
        closure.stop_class().closure_identity().as_str(),
        closure.stop_class().closure_digest()
    );
    assert_eq!(
        closure.session_label().closure_identity().as_str(),
        closure.session_label().closure_digest()
    );
    assert_eq!(
        closure.closure_identity().as_str(),
        closure.closure_digest()
    );
    assert!(!closure.closure_digest().is_empty());
    assert!(identity_boundary_hostile_matrix_artifact().certified());
}

#[test]
fn support_report_closed_requires_milestone_nine_six_certification_gate() {
    assert!(
        milestone_nine_six_certification_gate_certified(),
        "Closed posture requires named 9.6 suites registered with embedded sources"
    );
    let report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = report.identity_boundary_closure();
    assert_eq!(closure.status(), ForgeQueryMilestoneClosureStatus::Closed);
    assert!(closure.certification_gate_certified());
}

#[test]
fn support_report_keeps_identity_boundary_partial_when_ordinary_surface_is_disabled() {
    let report = query_disabled_application_facade().support_report();
    let closure = report.identity_boundary_closure();

    assert_eq!(closure.status(), ForgeQueryMilestoneClosureStatus::Partial);
    assert_eq!(
        closure.evidence_identity().status(),
        ForgeQueryMilestoneClosureStatus::Partial
    );
    assert_eq!(
        closure.stop_class().status(),
        ForgeQueryMilestoneClosureStatus::Partial
    );
    assert_eq!(
        closure.session_label().status(),
        ForgeQueryMilestoneClosureStatus::Partial
    );
    assert!(closure.residue_status().is_zero());
}

#[test]
fn support_report_digest_tracks_identity_boundary_publication_and_status() {
    let admitted_report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let denied_report = query_disabled_application_facade().support_report();

    assert_ne!(
        admitted_report.report_digest(),
        denied_report.report_digest(),
        "support report digest must include identity-boundary publication"
    );
    assert_ne!(
        admitted_report.identity_boundary_closure().closure_digest(),
        denied_report.identity_boundary_closure().closure_digest(),
        "identity-boundary evidence must track support posture changes"
    );
    assert_ne!(
        admitted_report.identity_boundary_closure().status(),
        denied_report.identity_boundary_closure().status(),
        "identity-boundary status must track whether the ordinary surface is actually available"
    );
}
