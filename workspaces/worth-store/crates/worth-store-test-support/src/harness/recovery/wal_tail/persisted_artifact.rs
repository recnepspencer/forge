use std::path::{Path, PathBuf};

pub fn prepare_persisted_wal_frame(
    family_root: &Path,
    segment: u64,
    start_lsn: u64,
    end_lsn: u64,
    subject: &str,
    payload: &[u8],
) -> (PathBuf, Vec<u8>) {
    let plan = worth_store_wal::prepare_wal_frame_append(
        family_root,
        segment,
        1,
        start_lsn,
        end_lsn,
        subject,
        payload,
    )
    .expect("valid persisted WAL fixture declaration");
    (
        family_root.join(plan.relative_path()),
        plan.encoded_frame().to_vec(),
    )
}
