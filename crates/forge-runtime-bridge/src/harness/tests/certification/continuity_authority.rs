use crate::facade::{
    BridgeContinuityAuthorityBasis, BridgeHistoricalLineageAuthority,
    BridgeHistoricalResolvedLineageIdentity, BridgeHistoricalResolvedRecordIdentity,
    TruthBranchIdentity, TruthSnapshotIdentity,
};

pub(crate) fn continuity_authority(
    branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
) -> BridgeHistoricalLineageAuthority {
    continuity_authority_with_successor(
        branch_identity,
        snapshot_identity,
        BridgeHistoricalResolvedRecordIdentity::new("entity:0:4:2"),
    )
}

pub(crate) fn continuity_authority_with_successor(
    branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    successor: BridgeHistoricalResolvedRecordIdentity,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(branch_identity, snapshot_identity),
        vec![BridgeHistoricalResolvedLineageIdentity::new(
            "lineage:test-successor",
        )],
        vec![successor],
        vec![7],
    )
    .expect("continuity authority should be canonical")
}

pub(crate) fn ambiguous_continuity_authority(
    branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(branch_identity, snapshot_identity),
        vec![
            BridgeHistoricalResolvedLineageIdentity::new("lineage:test-a"),
            BridgeHistoricalResolvedLineageIdentity::new("lineage:test-b"),
            BridgeHistoricalResolvedLineageIdentity::new("lineage:test-c"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::new("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::new("entity:0:5:2"),
        ],
        vec![7, 8, 9],
    )
    .expect("ambiguous continuity authority should be canonical")
}
