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
        BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
    )
}

pub(crate) fn continuity_authority_with_successor(
    branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    successor: BridgeHistoricalResolvedRecordIdentity,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(branch_identity, snapshot_identity),
        vec![BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned(
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
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:test-a"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:test-b"),
            BridgeHistoricalResolvedLineageIdentity::admit_bridge_owned("lineage:test-c"),
        ],
        vec![
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:4:2"),
            BridgeHistoricalResolvedRecordIdentity::admit_bridge_owned("entity:0:5:2"),
        ],
        vec![7, 8, 9],
    )
    .expect("ambiguous continuity authority should be canonical")
}
