use forge_query::facade::ForgeQuerySnapshotIdentity;

fn main() {
    let _snapshot = ForgeQuerySnapshotIdentity::from_external_authority_label("snapshot:test");
}
