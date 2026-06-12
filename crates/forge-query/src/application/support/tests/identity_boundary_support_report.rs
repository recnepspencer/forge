use super::query_disabled_application_facade;
use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryFolkloreResidueStatus, ForgeQueryMilestoneClosureStatus,
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
        closure.stop_class().status(),
        ForgeQueryMilestoneClosureStatus::Closed
    );
    assert_eq!(
        closure.stop_class().accessor(),
        "ForgeQueryRuntimeError::stop_class()"
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
        closure.residue_status(),
        ForgeQueryFolkloreResidueStatus::ZeroFolkloreResidue
    );
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
