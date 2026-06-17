use forge_query::facade::{ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity};

fn main() {
    let _commit = ForgeQueryCommitIdentity::from_external_authority_label("commit:test");
    let _snapshot = ForgeQuerySnapshotIdentity::from_external_authority_label("snapshot:test");
    let _entity = ForgeQueryEntityIdentity::authored_command("entity:test");
}
