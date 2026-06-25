use forge_query::facade::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQueryMutationDelta,
    ForgeQueryMutationKind, ForgeQueryMutationReceipt, ForgeQuerySnapshotIdentity,
};
use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
};

fn main() {
    let _receipt = ForgeQueryMutationReceipt::from_authoritative_parts(
        ForgeQueryCommitIdentity::from_relational_commit_id(1),
        ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        ),
        vec![ForgeQueryMutationDelta::from_touched_aspects(
            "TopologyEntity",
            ForgeQueryEntityIdentity::from_relational_record(
                RelationalBridgeRecordIdentityParts::entity(1, 1, 1),
            ),
            ForgeQueryMutationKind::Updated,
            vec![],
        )],
    );
}
