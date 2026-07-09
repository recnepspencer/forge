use rusqlite::Connection;

pub fn corrupt_local_file_commit_digest(path: &std::path::Path) {
    let raw = std::fs::read_to_string(path).unwrap();
    let persisted: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let digests = persisted
        .get("authoritative_artifact_digests")
        .and_then(serde_json::Value::as_object)
        .unwrap();
    let commit_digest_key = digests
        .keys()
        .find(|key| key.starts_with("CommitEnvelope:commit:"))
        .cloned()
        .unwrap();
    let artifact_digest = digests[&commit_digest_key]
        .get("artifact_digest")
        .and_then(serde_json::Value::as_str)
        .unwrap();
    let corrupted = raw.replacen(
        &format!("\"artifact_digest\": \"{artifact_digest}\""),
        "\"artifact_digest\": \"corrupted-digest\"",
        1,
    );
    std::fs::write(path, corrupted).unwrap();
}

pub fn corrupt_local_file_branch_head_digest(path: &std::path::Path) {
    let raw = std::fs::read_to_string(path).unwrap();
    let corrupted = raw.replacen(
        "\"head_commit_digest\": \"",
        "\"head_commit_digest\": \"drifted-",
        1,
    );
    std::fs::write(path, corrupted).unwrap();
}

pub fn corrupt_sqlite_authoritative_digest(path: &std::path::Path) {
    let connection = Connection::open(path).unwrap();
    connection
        .execute(
            "
            UPDATE authoritative_artifact_digests
            SET artifact_digest = 'corrupted-digest'
            WHERE rowid = (
                SELECT rowid
                FROM authoritative_artifact_digests
                WHERE artifact_family = 'CommitEnvelope'
                LIMIT 1
            )
            ",
            [],
        )
        .unwrap();
}

pub fn corrupt_sqlite_branch_head_digest(path: &std::path::Path) {
    let connection = Connection::open(path).unwrap();
    connection
        .execute(
            "
            UPDATE branch_head_records
            SET head_commit_digest = 'drifted-digest'
            WHERE rowid = (
                SELECT rowid
                FROM branch_head_records
                LIMIT 1
            )
            ",
            [],
        )
        .unwrap();
}

pub fn delete_sqlite_parent_row(path: &std::path::Path) {
    let connection = Connection::open(path).unwrap();
    connection
        .execute(
            "
            DELETE FROM commit_parent_records
            WHERE rowid = (
                SELECT rowid
                FROM commit_parent_records
                LIMIT 1
            )
            ",
            [],
        )
        .unwrap();
}

pub fn corrupt_sqlite_envelope_payload(path: &std::path::Path) {
    let connection = Connection::open(path).unwrap();
    connection
        .execute(
            "
            UPDATE commit_envelopes
            SET envelope_payload = '{not-json'
            WHERE rowid = (
                SELECT rowid
                FROM commit_envelopes
                LIMIT 1
            )
            ",
            [],
        )
        .unwrap();
}
