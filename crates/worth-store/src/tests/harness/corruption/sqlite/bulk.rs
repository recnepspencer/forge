use super::*;

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
    payload["checkpoint"]["next_chunk_ordinal"] = serde_json::json!(completed_chunk_ordinal + 1);
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
                serde_json::to_string(&payload)
                    .expect("sqlite bulk checkpoint payload should encode")
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
    payload["index"]["highest_committed_chunk_ordinal"] =
        serde_json::json!(regressed_chunk_ordinal);
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

pub fn drift_sqlite_frozen_transform_partition_payload_member_width(
    path: &std::path::Path,
    program_id: &str,
    partition_digest: &str,
) {
    let connection = rusqlite::Connection::open(path).expect("sqlite database should open");
    let payload_json: String = connection
        .query_row(
            "
            SELECT payload_json
            FROM frozen_transform_partition_records
            WHERE program_id = ?1 AND partition_digest = ?2
            ",
            rusqlite::params![program_id, partition_digest],
            |row| row.get::<_, String>(0),
        )
        .expect("sqlite frozen transform partition payload should exist");
    let mut payload: serde_json::Value = serde_json::from_str(&payload_json)
        .expect("sqlite frozen transform partition payload should decode");
    let width = payload["partition"]["ordered_members"][0]["width_units"]
        .as_u64()
        .expect("sqlite frozen transform partition width should exist");
    payload["partition"]["ordered_members"][0]["width_units"] = serde_json::json!(width + 1);
    connection
        .execute(
            "
            UPDATE frozen_transform_partition_records
            SET payload_json = ?3
            WHERE program_id = ?1 AND partition_digest = ?2
            ",
            rusqlite::params![
                program_id,
                partition_digest,
                serde_json::to_string(&payload)
                    .expect("sqlite frozen transform partition payload should encode")
            ],
        )
        .expect("sqlite frozen transform partition payload should be updated");
}
