use crate::consumer_kit::support_pinning::{
    support_pinning_contract, ForgeQueryPinnedSupportStatus, ForgeQueryPinnedTeachingPosture,
    ForgeQuerySupportPinningErrorKind,
};
use crate::runtime::ForgeQueryRuntimeFacadeFamily;

use super::scaffold_snapshot;

#[test]
fn declaration_rejects_missing_required_posture_before_sealing() {
    let snapshot = scaffold_snapshot();

    let error = support_pinning_contract("worth-kernel")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .bind_live_row_digest()
        })
        .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQuerySupportPinningErrorKind::MissingRequiredTeachingPosture
    );
    assert_eq!(error.family(), Some("write"));
}

#[test]
fn declaration_rejects_missing_live_digest_binding_before_sealing() {
    let snapshot = scaffold_snapshot();

    let error = support_pinning_contract("worth-kernel")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
        })
        .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQuerySupportPinningErrorKind::MissingLiveRowDigestBinding
    );
    assert_eq!(error.family(), Some("write"));
}
