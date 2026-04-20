pub fn simulate_legacy_milestone_6_commit_coupled_layout_seed_storage(path: &std::path::Path) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS milestone_6_published_layout_request_records (
                artifact_id TEXT PRIMARY KEY,
                branch_id TEXT NOT NULL,
                frontier_commit_id TEXT NOT NULL,
                scope_class TEXT NOT NULL,
                payload_json TEXT NOT NULL
            )
            ",
            [],
        )
        .expect("legacy milestone 6 seed table should exist");
    connection
        .execute(
            "
            INSERT OR REPLACE INTO milestone_6_published_layout_request_records(
                artifact_id,
                branch_id,
                frontier_commit_id,
                scope_class,
                payload_json
            )
            SELECT artifact_id, branch_id, frontier_commit_id, scope_class, payload_json
            FROM milestone_6_commit_coupled_layout_seed_records
            ",
            [],
        )
        .expect("legacy milestone 6 seed rows should copy");
    connection
        .execute(
            "DELETE FROM milestone_6_commit_coupled_layout_seed_records",
            [],
        )
        .expect("new milestone 6 seed rows should clear");
}

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

