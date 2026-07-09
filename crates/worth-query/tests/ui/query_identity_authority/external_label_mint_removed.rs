use worth_query::facade::{WorthQueryCommitIdentity, WorthQueryEntityIdentity, WorthQuerySnapshotIdentity};

fn main() {
    let _commit = WorthQueryCommitIdentity::from_external_authority_label("commit:test");
    let _snapshot = WorthQuerySnapshotIdentity::from_external_authority_label("snapshot:test");
    let _entity = WorthQueryEntityIdentity::authored_command("entity:test");
}
