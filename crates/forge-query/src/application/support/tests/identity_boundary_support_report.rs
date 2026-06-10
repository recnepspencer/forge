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
        closure.evidence_identity().covered_surfaces(),
        &[
            "runtime_public_support_matrix_row",
            "runtime_public_support_matrix",
            "runtime_public_api_family_contract",
            "runtime_public_api_contract",
            "runtime_public_api_transcript_evidence",
            "runtime_state_snapshot",
            "preview_basis_admission",
            "branch_basis_admission",
            "preview_intent_admission",
            "preview_intent_receipt",
            "branch_intent_admission",
            "branch_intent_receipt",
            "intent_denial_evidence",
            "application_support_report",
        ]
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
        &[
            "typed-family-admission-denial",
            "typed-preview-promotion-stop",
            "typed-session-label-collision-stop",
        ]
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
        &[
            "runtime.preview",
            "runtime.branch",
            "runtime.try_preview",
            "runtime.try_branch",
            "workspace.preview",
            "workspace.branch",
        ]
    );
    assert_eq!(
        closure.residue_status(),
        ForgeQueryFolkloreResidueStatus::ZeroFolkloreResidue
    );
    assert_eq!(
        closure.exact_zero_format_digest_paths(),
        &[
            "application/support/report.rs",
            "runtime/support_matrix.rs",
            "runtime/state_snapshot.rs",
            "runtime/public_api_transcript.rs",
        ]
    );
    assert_eq!(
        closure.exact_zero_string_matching_paths(),
        &["runtime/tests/stop_class/consumer_support/routing.rs",]
    );
    assert_eq!(
        closure.exact_zero_raw_session_admission_paths(),
        &["runtime/runtime_sessions.rs", "runtime/workspace.rs",]
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
