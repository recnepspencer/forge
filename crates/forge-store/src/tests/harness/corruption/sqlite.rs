use crate::bulk::compute_checkpoint_digest;

pub fn corrupt_first_sqlite_wal_record_digest(path: &std::path::Path) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            UPDATE wal_records
            SET record_digest = 'corrupted-wal-digest'
            WHERE wal_sequence = (
                SELECT wal_sequence
                FROM wal_records
                ORDER BY wal_sequence
                LIMIT 1
            )
            ",
            [],
        )
        .expect("sqlite wal digest should be corrupted");
}

pub fn corrupt_first_sqlite_snapshot_image(path: &std::path::Path) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            UPDATE snapshot_image_records
            SET image_payload = '{\"corrupted\":true}'
            WHERE snapshot_id = (
                SELECT snapshot_id
                FROM snapshot_image_records
                ORDER BY snapshot_id
                LIMIT 1
            )
            ",
            [],
        )
        .expect("sqlite snapshot image payload should be corrupted");
}

pub fn delete_first_sqlite_snapshot_image(path: &std::path::Path) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            DELETE FROM snapshot_image_records
            WHERE snapshot_id = (
                SELECT snapshot_id
                FROM snapshot_image_records
                ORDER BY snapshot_id
                LIMIT 1
            )
            ",
            [],
        )
        .expect("sqlite snapshot image row should be deleted");
}

pub fn corrupt_first_sqlite_snapshot_basis_version(path: &std::path::Path) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            UPDATE snapshot_basis_records
            SET snapshot_family_version = 999
            WHERE snapshot_id = (
                SELECT snapshot_id
                FROM snapshot_basis_records
                ORDER BY snapshot_id
                LIMIT 1
            )
            ",
            [],
        )
        .expect("sqlite snapshot basis version should be corrupted");
}

pub fn delete_first_sqlite_lineage_support_record(path: &std::path::Path) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            DELETE FROM lineage_support_records
            WHERE artifact_id = (
                SELECT artifact_id
                FROM lineage_support_records
                ORDER BY commit_id
                LIMIT 1
            )
            ",
            [],
        )
        .expect("sqlite lineage support row should be deleted");
}

pub fn delete_sqlite_bulk_checkpoint(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    checkpoint_sequence: u64,
) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            DELETE FROM bulk_progress_checkpoint_records
            WHERE program_id = ?1 AND plan_id = ?2 AND checkpoint_sequence = ?3
            ",
            rusqlite::params![program_id, plan_id, checkpoint_sequence],
        )
        .expect("sqlite bulk checkpoint row should be deleted");
}

pub fn regress_sqlite_bulk_checkpoint_completed_chunk(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    checkpoint_sequence: u64,
    completed_chunk_ordinal: u64,
) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    let mut statement = connection
        .prepare(
            "
            SELECT payload_json
            FROM bulk_progress_checkpoint_records
            WHERE program_id = ?1 AND plan_id = ?2 AND checkpoint_sequence = ?3
            ",
        )
        .expect("sqlite bulk checkpoint query should prepare");
    let payload_json: String = statement
        .query_row(
            rusqlite::params![program_id, plan_id, checkpoint_sequence],
            |row| row.get(0),
        )
        .expect("sqlite bulk checkpoint payload should exist");
    let mut payload: serde_json::Value =
        serde_json::from_str(&payload_json).expect("sqlite bulk checkpoint payload should decode");
    payload["checkpoint"]["completed_chunk_ordinal"] = serde_json::json!(completed_chunk_ordinal);
    payload["checkpoint"]["next_chunk_ordinal"] =
        serde_json::json!(completed_chunk_ordinal + 1);
    let witness_artifact_id = payload["checkpoint"]["last_committed_chunk_witness_artifact_id"]
        .as_str()
        .expect("sqlite bulk checkpoint witness artifact id should be present");
    let checkpoint_digest = compute_checkpoint_digest(
        program_id,
        plan_id,
        checkpoint_sequence,
        crate::ChunkOrdinal::new(completed_chunk_ordinal),
        crate::ChunkOrdinal::new(completed_chunk_ordinal + 1),
        witness_artifact_id,
    );
    payload["checkpoint"]["checkpoint_digest"] = serde_json::json!(checkpoint_digest);
    connection
        .execute(
            "
            UPDATE bulk_progress_checkpoint_records
            SET payload_json = ?4
            WHERE program_id = ?1 AND plan_id = ?2 AND checkpoint_sequence = ?3
            ",
            rusqlite::params![
                program_id,
                plan_id,
                checkpoint_sequence,
                serde_json::to_string(&payload).expect("sqlite bulk checkpoint payload should encode")
            ],
        )
        .expect("sqlite bulk checkpoint payload should be updated");
}

pub fn regress_sqlite_bulk_witness_index_highest_ordinal(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    regressed_chunk_ordinal: u64,
) {
    let connection = rusqlite::Connection::open(path).expect("sqlite database should open");
    let payload_json: String = connection
        .query_row(
            "
            SELECT payload_json
            FROM program_chunk_witness_index_records
            WHERE program_id = ?1 AND plan_id = ?2
            ",
            rusqlite::params![program_id, plan_id],
            |row| row.get::<_, String>(0),
        )
        .expect("sqlite bulk witness index payload should exist");
    let mut payload: serde_json::Value = serde_json::from_str(&payload_json)
        .expect("sqlite bulk witness index payload should decode");
    payload["index"]["highest_committed_chunk_ordinal"] = serde_json::json!(regressed_chunk_ordinal);
    connection
        .execute(
            "
            UPDATE program_chunk_witness_index_records
            SET payload_json = ?3
            WHERE program_id = ?1 AND plan_id = ?2
            ",
            rusqlite::params![
                program_id,
                plan_id,
                serde_json::to_string(&payload)
                    .expect("sqlite bulk witness index payload should encode")
            ],
        )
        .expect("sqlite bulk witness index payload should be updated");
}

pub fn drift_sqlite_bulk_witness_index_witness_count(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    drifted_witness_count: u64,
) {
    let connection = rusqlite::Connection::open(path).expect("sqlite database should open");
    let payload_json: String = connection
        .query_row(
            "
            SELECT payload_json
            FROM program_chunk_witness_index_records
            WHERE program_id = ?1 AND plan_id = ?2
            ",
            rusqlite::params![program_id, plan_id],
            |row| row.get::<_, String>(0),
        )
        .expect("sqlite bulk witness index payload should exist");
    let mut payload: serde_json::Value = serde_json::from_str(&payload_json)
        .expect("sqlite bulk witness index payload should decode");
    payload["index"]["witness_count"] = serde_json::json!(drifted_witness_count);
    connection
        .execute(
            "
            UPDATE program_chunk_witness_index_records
            SET payload_json = ?3
            WHERE program_id = ?1 AND plan_id = ?2
            ",
            rusqlite::params![
                program_id,
                plan_id,
                serde_json::to_string(&payload)
                    .expect("sqlite bulk witness index payload should encode")
            ],
        )
        .expect("sqlite bulk witness index payload should be updated");
}
