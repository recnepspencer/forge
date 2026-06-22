use forge_query::facade::{
    ForgeQueryRuntimeSnapshotIdentityAdapter, ForgeQuerySnapshotIdentity,
};

struct CurrentSnapshotIdentity;

impl ForgeQueryRuntimeSnapshotIdentityAdapter for CurrentSnapshotIdentity {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        ForgeQuerySnapshotIdentity::empty_relational_state()
    }
}

fn main() {}
