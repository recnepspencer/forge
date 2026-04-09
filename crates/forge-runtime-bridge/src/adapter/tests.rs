use std::sync::Arc;

use crate::continuity::BridgeContinuityAuthorityBasis;
use crate::input::envelope::TruthBranchIdentity;
use crate::snapshot::TruthSnapshotIdentity;

use super::BridgeHistoricalLineageAuthority;

#[test]
fn historical_lineage_authority_digest_is_canonical_for_same_inputs() {
    let authority_basis = BridgeContinuityAuthorityBasis::new(
        TruthBranchIdentity::new("main"),
        TruthSnapshotIdentity::new("snapshot-a"),
    );

    let left = BridgeHistoricalLineageAuthority::try_new(
        authority_basis.clone(),
        vec![Arc::from("lineage:1"), Arc::from("lineage:2")],
        vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
        vec![3, 7],
    )
    .expect("canonical lineage authority should build");
    let right = BridgeHistoricalLineageAuthority::try_new(
        authority_basis,
        vec![Arc::from("lineage:1"), Arc::from("lineage:2")],
        vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
        vec![3, 7],
    )
    .expect("canonical lineage authority should build");

    assert_eq!(left, right);
    assert!(left
        .lineage_digest()
        .starts_with("historical-lineage-authority:sha256:"));
}

#[test]
fn historical_lineage_authority_rejects_noncanonical_inputs() {
    let authority_basis = BridgeContinuityAuthorityBasis::new(
        TruthBranchIdentity::new("main"),
        TruthSnapshotIdentity::new("snapshot-a"),
    );

    let error = BridgeHistoricalLineageAuthority::try_new(
        authority_basis,
        vec![Arc::from("lineage:2"), Arc::from("lineage:1")],
        vec![Arc::from("entity:0:5:2"), Arc::from("entity:0:4:2")],
        vec![7, 3],
    )
    .expect_err("noncanonical lineage authority should be rejected");

    assert!(error.to_string().contains("canonical order"));
}
