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
