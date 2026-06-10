use super::query_disabled_application_facade;
use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryFolkloreResidueStatus, ForgeQueryMilestoneClosureStatus,
    EXACT_ZERO_FORMAT_DIGEST_PATHS, EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS,
    EXACT_ZERO_STRING_MATCHING_PATHS, EVIDENCE_IDENTITY_COVERED_SURFACES,
    SESSION_LABEL_ORDINARY_ENTRYPOINTS, STOP_CLASS_COVERED_CONTRACTS,
};
use crate::ForgeQueryEvidenceIdentityScheme;

#[test]
fn support_report_publishes_closed_identity_boundary() {
    let report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = report.identity_boundary_closure();

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
    assert!(!closure.hostile_matrix_digest().is_empty());
    assert!(!closure.closure_digest().is_empty());
}

#[test]
fn support_report_digest_tracks_identity_boundary_publication() {
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
}
