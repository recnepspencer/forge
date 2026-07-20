use crate::consumer_kit::support_pinning::{
    support_pinning_contract, WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture,
    WorthQuerySupportPinningErrorKind,
};
use crate::runtime::WorthQueryRuntimeFacadeFamily;

use super::scaffold_snapshot;

#[test]
fn declaration_rejects_missing_required_posture_before_sealing() {
    let snapshot = scaffold_snapshot();

    let error = support_pinning_contract("worth-kernel")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(WorthQueryRuntimeFacadeFamily::Write, |row| {
            row.status(WorthQueryPinnedSupportStatus::Supported)
                .bind_live_row_digest()
        })
        .unwrap_err();

    assert_eq!(
        error.kind(),
        WorthQuerySupportPinningErrorKind::MissingRequiredTeachingPosture
    );
    assert_eq!(error.family(), Some("write"));
}

#[test]
fn declaration_rejects_missing_live_digest_binding_before_sealing() {
    let snapshot = scaffold_snapshot();

    let error = support_pinning_contract("worth-kernel")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(WorthQueryRuntimeFacadeFamily::Write, |row| {
            row.status(WorthQueryPinnedSupportStatus::Supported)
                .teaching_posture(WorthQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
        })
        .unwrap_err();

    assert_eq!(
        error.kind(),
        WorthQuerySupportPinningErrorKind::MissingLiveRowDigestBinding
    );
    assert_eq!(error.family(), Some("write"));
}
