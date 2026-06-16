use crate::continuity::BridgeContinuityAuthorityBasis;
use crate::error::BridgeLineageSourceErrorKind;

use super::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalResolvedLineageIdentity,
    BridgeHistoricalResolvedRecordIdentity,
};

fn authority_basis() -> BridgeContinuityAuthorityBasis {
    BridgeContinuityAuthorityBasis::new(
        crate::truth_identity_fixtures::truth_branch_fixture("main"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
    )
}

#[test]
fn historical_lineage_authority_digest_is_canonical_for_same_inputs() {
    let authority_basis = authority_basis();

    let left = BridgeHistoricalLineageAuthority::try_new(
        authority_basis.clone(),
        vec![
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:2"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:5:2"),
        ],
        vec![3, 7],
    )
    .expect("canonical lineage authority should build");
    let right = BridgeHistoricalLineageAuthority::try_new(
        authority_basis,
        vec![
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:2"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:5:2"),
        ],
        vec![3, 7],
    )
    .expect("canonical lineage authority should build");

    assert_eq!(left, right);
    assert!(left
        .lineage_digest()
        .starts_with("historical-lineage-authority:sha256:"));
}

#[test]
fn historical_lineage_authority_rejects_noncanonical_lineage_identities() {
    let error = BridgeHistoricalLineageAuthority::try_new(
        authority_basis(),
        vec![
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:2"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:5:2"),
        ],
        vec![3, 7],
    )
    .expect_err("noncanonical lineage authority should be rejected");

    assert_eq!(
        error.kind(),
        BridgeLineageSourceErrorKind::NonCanonicalResolvedLineageIdentities
    );
}

#[test]
fn historical_lineage_authority_rejects_noncanonical_record_identities() {
    let error = BridgeHistoricalLineageAuthority::try_new(
        authority_basis(),
        vec![
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:2"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:5:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
        ],
        vec![3, 7],
    )
    .expect_err("noncanonical record authority should be rejected");

    assert_eq!(
        error.kind(),
        BridgeLineageSourceErrorKind::NonCanonicalResolvedRecordIdentities
    );
}

#[test]
fn historical_lineage_authority_rejects_duplicate_lineage_identities() {
    let error = BridgeHistoricalLineageAuthority::try_new(
        authority_basis(),
        vec![
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:5:2"),
        ],
        vec![3, 7],
    )
    .expect_err("duplicate lineage authority should be rejected");

    assert_eq!(
        error.kind(),
        BridgeLineageSourceErrorKind::DuplicateResolvedLineageIdentities
    );
}

#[test]
fn historical_lineage_authority_rejects_duplicate_record_identities() {
    let error = BridgeHistoricalLineageAuthority::try_new(
        authority_basis(),
        vec![
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:2"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
        ],
        vec![3, 7],
    )
    .expect_err("duplicate record authority should be rejected");

    assert_eq!(
        error.kind(),
        BridgeLineageSourceErrorKind::DuplicateResolvedRecordIdentities
    );
}

#[test]
fn historical_lineage_authority_rejects_duplicate_event_ids() {
    let error = BridgeHistoricalLineageAuthority::try_new(
        authority_basis(),
        vec![
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:2"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:5:2"),
        ],
        vec![3, 3],
    )
    .expect_err("duplicate event authority should be rejected");

    assert_eq!(
        error.kind(),
        BridgeLineageSourceErrorKind::DuplicateTraversedEventIds
    );
}

#[test]
fn historical_lineage_authority_rejects_noncanonical_event_ids() {
    let error = BridgeHistoricalLineageAuthority::try_new(
        authority_basis(),
        vec![
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:1"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:2"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:5:2"),
        ],
        vec![7, 3],
    )
    .expect_err("noncanonical event authority should be rejected");

    assert_eq!(
        error.kind(),
        BridgeLineageSourceErrorKind::NonCanonicalTraversedEventIds
    );
}
