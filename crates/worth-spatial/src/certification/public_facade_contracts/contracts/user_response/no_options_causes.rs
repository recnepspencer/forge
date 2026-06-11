use worth_spatial::facade::user_response::{
    WorthNoOptionsCause, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

use super::contract_subject::{dirty_input_response, missing_evidence_response};

#[test]
fn user_response_distinguishes_dirty_input_from_missing_evidence_no_options() {
    let dirty = dirty_input_response("user-response-no-options-dirty");
    let missing = missing_evidence_response("user-response-no-options-missing-evidence");

    assert_eq!(dirty.outcome().kind(), WorthUserOutcomeKind::NoOptions);
    assert_eq!(
        dirty.outcome().cause().map(|cause| cause.kind()),
        Some(WorthUserOutcomeCauseKind::DirtyInput)
    );
    assert_eq!(
        dirty
            .outcome()
            .cause()
            .and_then(|cause| cause.no_options_cause()),
        Some(WorthNoOptionsCause::DirtyInput)
    );
    assert!(dirty.outcome().choices().is_empty());
    assert!(dirty
        .human_response()
        .summary()
        .contains("inspect topology"));

    assert_eq!(missing.outcome().kind(), WorthUserOutcomeKind::NoOptions);
    assert_eq!(
        missing.outcome().cause().map(|cause| cause.kind()),
        Some(WorthUserOutcomeCauseKind::MissingEvidence)
    );
    assert_eq!(
        missing
            .outcome()
            .cause()
            .and_then(|cause| cause.no_options_cause()),
        Some(WorthNoOptionsCause::MissingEvidence)
    );
    assert!(missing.outcome().choices().is_empty());
    assert!(missing
        .human_response()
        .summary()
        .contains("retained artifact"));
    assert_ne!(
        missing.evidence().digest(),
        missing.human_response().summary()
    );
    assert_eq!(
        missing.evidence().digest(),
        missing.evidence().source_identity()
    );
    assert_eq!(
        missing.stage_identity().upstream_receipt(),
        missing.evidence().source_identity()
    );
}
