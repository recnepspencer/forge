use worth_query::facade::foundation::{WorthQueryEntityIdentity, WorthQuerySnapshotIdentity};

fn inspect(entity: &WorthQueryEntityIdentity, snapshot: &WorthQuerySnapshotIdentity) {
    let _ = entity.relational_record_parts();
    let _ = snapshot.relational_parts();
}

fn main() {}
